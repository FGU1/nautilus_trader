// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2025 Nautech Systems Pty Ltd. All rights reserved.
//  https://nautechsystems.io
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

//! Utilities for transferring heap-allocated Rust `Vec<T>` values across an FFI boundary.
//!
//! The primary abstraction offered by this module is `CVec`, a C-compatible struct that stores
//! a raw pointer (`ptr`) together with the vector’s logical `len` and `cap`.  By moving the
//! allocation metadata into a plain `repr(C)` type we allow the memory created by Rust to be
//! owned, inspected, and ultimately freed by foreign code (or vice-versa) without introducing
//! undefined behaviour.
//!
//! Only a very small API surface is exposed to C:
//!
//! * `cvec_new` – create an empty vector representation.
//! * `cvec_drop` – free a vector that was obtained from Rust.
//!
//! All other manipulation happens on the Rust side before relinquishing ownership.  This keeps the
//! rules for memory safety straightforward: foreign callers must treat the memory region pointed
//! to by `ptr` as *opaque* and interact with it solely through the functions provided here.

use std::{ffi::c_void, fmt::Display, ptr::null};

/// `CVec` is a C compatible struct that stores an opaque pointer to a block of
/// memory, it's length and the capacity of the vector it was allocated from.
///
/// # Safety
///
/// Changing the values here may lead to undefined behavior when the memory is dropped.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CVec {
    /// Opaque pointer to block of memory storing elements to access the
    /// elements cast it to the underlying type.
    pub ptr: *mut c_void,
    /// The number of elements in the block.
    pub len: usize,
    /// The capacity of vector from which it was allocated.
    /// Used when deallocating the memory
    pub cap: usize,
}

/// Empty derivation for Send to satisfy `pyclass` requirements
/// however this is only designed for single threaded use for now
unsafe impl Send for CVec {}

impl CVec {
    /// Returns an empty [`CVec`].
    ///
    /// This is primarily useful for constructing a sentinel value that represents the
    /// absence of data when crossing the FFI boundary.
    #[must_use]
    pub const fn empty() -> Self {
        Self {
            // Explicitly type cast the pointer to some type to satisfy the
            // compiler. Since the pointer is null it works for any type.
            ptr: null::<bool>() as *mut c_void,
            len: 0,
            cap: 0,
        }
    }
}

/// Consumes and leaks the Vec, returning a mutable pointer to the contents as
/// a [`CVec`]. The memory has been leaked and now exists for the lifetime of the
/// program unless dropped manually.
/// Note: drop the memory by reconstructing the vec using `from_raw_parts` method
/// as shown in the test below.
impl<T> From<Vec<T>> for CVec {
    fn from(mut data: Vec<T>) -> Self {
        if data.is_empty() {
            Self::empty()
        } else {
            let len = data.len();
            let cap = data.capacity();
            let ptr = data.as_mut_ptr();
            std::mem::forget(data);
            Self {
                ptr: ptr.cast::<std::ffi::c_void>(),
                len,
                cap,
            }
        }
    }
}

impl Display for CVec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CVec {{ ptr: {:?}, len: {}, cap: {} }}",
            self.ptr, self.len, self.cap,
        )
    }
}

////////////////////////////////////////////////////////////////////////////////
// C API
////////////////////////////////////////////////////////////////////////////////
/// Free the heap allocation represented by `cvec`.
///
/// # Safety
///
/// The pointer **must** either originate from the Rust side through the `From<Vec<T>>`
/// implementation or be the return value of one of the exported functions in this module.  It is
/// undefined behaviour to pass an arbitrary or already-freed pointer.
#[cfg(feature = "ffi")]
#[unsafe(no_mangle)]
pub extern "C" fn cvec_drop(cvec: CVec) {
    let CVec { ptr, len, cap } = cvec;

    // SAFETY: CVec currently only supports u8 data through FFI.
    // The generic From<Vec<T>> implementation should only be used internally
    // where the caller ensures proper type-matched deallocation.
    // For FFI boundaries, we standardize on u8 to avoid type confusion.
    let data: Vec<u8> = unsafe { Vec::from_raw_parts(ptr.cast::<u8>(), len, cap) };
    drop(data); // Memory freed here
}

/// Construct a new *empty* [`CVec`] value for use as initialiser/sentinel in foreign code.
#[cfg(feature = "ffi")]
#[unsafe(no_mangle)]
pub const extern "C" fn cvec_new() -> CVec {
    CVec::empty()
}

#[cfg(test)]
mod tests {
    use rstest::*;

    use super::CVec;

    /// Access values from a vector converted into a [`CVec`].
    #[rstest]
    #[allow(unused_assignments)]
    fn access_values_test() {
        let test_data = vec![1_u64, 2, 3];
        let mut vec_len = 0;
        let mut vec_cap = 0;
        let cvec: CVec = {
            let data = test_data.clone();
            vec_len = data.len();
            vec_cap = data.capacity();
            data.into()
        };

        let CVec { ptr, len, cap } = cvec;
        assert_eq!(len, vec_len);
        assert_eq!(cap, vec_cap);

        let data = ptr.cast::<u64>();
        unsafe {
            assert_eq!(*data, test_data[0]);
            assert_eq!(*data.add(1), test_data[1]);
            assert_eq!(*data.add(2), test_data[2]);
        }

        unsafe {
            // reconstruct the struct and drop the memory to deallocate
            let _ = Vec::from_raw_parts(ptr.cast::<u64>(), len, cap);
        }
    }

    /// After deallocating the vector the block of memory may not
    /// contain the same values.
    #[rstest]
    #[ignore = "Flaky on some platforms"]
    fn drop_test() {
        let test_data = vec![1, 2, 3];
        let cvec: CVec = {
            let data = test_data.clone();
            data.into()
        };

        let CVec { ptr, len, cap } = cvec;
        let data = ptr.cast::<u64>();

        unsafe {
            let data: Vec<u64> = Vec::from_raw_parts(ptr.cast::<u64>(), len, cap);
            drop(data);
        }

        unsafe {
            assert_ne!(*data, test_data[0]);
            assert_ne!(*data.add(1), test_data[1]);
            assert_ne!(*data.add(2), test_data[2]);
        }
    }

    /// An empty vector gets converted to a null pointer wrapped in a [`CVec`].
    #[rstest]
    fn empty_vec_should_give_null_ptr() {
        let data: Vec<u64> = vec![];
        let cvec: CVec = data.into();
        assert_eq!(cvec.ptr.cast::<u64>(), std::ptr::null_mut::<u64>());
    }
}
