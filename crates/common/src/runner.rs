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

use std::{
    cell::{OnceCell, RefCell},
    collections::VecDeque,
    rc::Rc,
};

use crate::{
    clock::Clock,
    messages::{DataEvent, data::DataCommand},
    timer::TimeEvent,
};

pub type GlobalClock = Rc<RefCell<dyn Clock>>;

/// # Panics
///
/// Panics if thread-local storage cannot be accessed or the global clock is uninitialized.
#[must_use]
pub fn get_global_clock() -> Rc<RefCell<dyn Clock>> {
    CLOCK
        .try_with(|clock| {
            clock
                .get()
                .expect("Clock should be initialized by runner")
                .clone()
        })
        .expect("Should be able to access thread local storage")
}

/// # Panics
///
/// Panics if thread-local storage cannot be accessed or the global clock is already set.
pub fn set_global_clock(c: Rc<RefCell<dyn Clock>>) {
    CLOCK
        .try_with(|clock| {
            assert!(clock.set(c).is_ok(), "Global clock already set");
        })
        .expect("Should be able to access thread local clock");
}

pub type DataCommandQueue = Rc<RefCell<VecDeque<DataCommand>>>;

/// Get globally shared message bus command queue
/// # Panics
///
/// Panics if thread-local storage cannot be accessed.
#[must_use]
pub fn get_data_cmd_queue() -> DataCommandQueue {
    DATA_CMD_QUEUE
        .try_with(std::clone::Clone::clone)
        .expect("Should be able to access thread local storage")
}

pub trait DataQueue {
    fn push(&mut self, event: DataEvent);
}

pub type GlobalDataQueue = Rc<RefCell<dyn DataQueue>>;

#[derive(Debug)]
pub struct SyncDataQueue(VecDeque<DataEvent>);

impl DataQueue for SyncDataQueue {
    fn push(&mut self, event: DataEvent) {
        self.0.push_back(event);
    }
}

/// # Panics
///
/// Panics if thread-local storage cannot be accessed or the data event queue is uninitialized.
#[must_use]
pub fn get_data_evt_queue() -> Rc<RefCell<dyn DataQueue>> {
    DATA_EVT_QUEUE
        .try_with(|dq| {
            dq.get()
                .expect("Data queue should be initialized by runner")
                .clone()
        })
        .expect("Should be able to access thread local storage")
}

/// # Panics
///
/// Panics if thread-local storage cannot be accessed or the global data event queue is already set.
pub fn set_data_evt_queue(dq: Rc<RefCell<dyn DataQueue>>) {
    DATA_EVT_QUEUE
        .try_with(|deque| {
            assert!(deque.set(dq).is_ok(), "Global data queue already set");
        })
        .expect("Should be able to access thread local storage");
}

thread_local! {
    static CLOCK: OnceCell<GlobalClock> = OnceCell::new();
    static DATA_EVT_QUEUE: OnceCell<GlobalDataQueue> = OnceCell::new();
    static DATA_CMD_QUEUE: DataCommandQueue = Rc::new(RefCell::new(VecDeque::new()));
}

// Represents different event types for the runner.
#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum RunnerEvent {
    Data(DataEvent),
    Timer(TimeEvent),
}
