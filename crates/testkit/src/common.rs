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

use std::path::PathBuf;

use nautilus_core::paths::get_test_data_path;

use crate::files::ensure_file_exists_or_download_http;

/// Returns the full path to the test data file at the specified relative `path` within the standard test data directory.
///
/// # Panics
///
/// Panics if the computed path cannot be represented as a valid UTF-8 string.
#[must_use]
pub fn get_test_data_file_path(path: &str) -> String {
    get_test_data_path()
        .join(path)
        .to_str()
        .unwrap()
        .to_string()
}

/// Returns the full path to the Nautilus-specific test data file given by `filename`, within the configured precision directory ("64-bit" or "128-bit").
///
/// # Panics
///
/// Panics if the computed path cannot be represented as a valid UTF-8 string.
#[must_use]
#[allow(unused_mut)]
pub fn get_nautilus_test_data_file_path(filename: &str) -> String {
    let mut path = get_test_data_path().join("nautilus");

    #[cfg(feature = "high-precision")]
    {
        path = path.join("128-bit");
    }
    #[cfg(not(feature = "high-precision"))]
    {
        path = path.join("64-bit");
    }

    path.join(filename).to_str().unwrap().to_string()
}

/// Returns the path to the checksums file for large test data files.
#[must_use]
pub fn get_test_data_large_checksums_filepath() -> PathBuf {
    get_test_data_path().join("large").join("checksums.json")
}

/// Ensures that the specified test data file exists locally by downloading it if necessary, using the provided `url`.
///
/// # Panics
///
/// Panics if the download or checksum verification fails, or if the resulting path cannot be represented as a valid UTF-8 string.
#[must_use]
pub fn ensure_test_data_exists(filename: &str, url: &str) -> PathBuf {
    let filepath = get_test_data_path().join("large").join(filename);
    let checksums_filepath = get_test_data_large_checksums_filepath();
    ensure_file_exists_or_download_http(&filepath, url, Some(&checksums_filepath)).unwrap();
    filepath
}

/// Ensures the Tardis Deribit incremental book L2 test data exists and returns its path.
#[must_use]
pub fn ensure_data_exists_tardis_deribit_book_l2() -> PathBuf {
    let filename = "tardis_deribit_incremental_book_L2_2020-04-01_BTC-PERPETUAL.csv.gz";
    let base_url = "https://datasets.tardis.dev";
    let url = format!("{base_url}/v1/deribit/incremental_book_L2/2020/04/01/BTC-PERPETUAL.csv.gz");
    ensure_test_data_exists(filename, &url)
}

/// Ensures the Tardis Binance Futures book snapshot (depth 5) test data exists and returns its path.
#[must_use]
pub fn ensure_data_exists_tardis_binance_snapshot5() -> PathBuf {
    let filename = "tardis_binance-futures_book_snapshot_5_2020-09-01_BTCUSDT.csv.gz";
    let base_url = "https://datasets.tardis.dev";
    let url = format!("{base_url}/v1/binance-futures/book_snapshot_5/2020/09/01/BTCUSDT.csv.gz");
    ensure_test_data_exists(filename, &url)
}

/// Ensures the Tardis Binance Futures book snapshot (depth 25) test data exists and returns its path.
#[must_use]
pub fn ensure_data_exists_tardis_binance_snapshot25() -> PathBuf {
    let filename = "tardis_binance-futures_book_snapshot_25_2020-09-01_BTCUSDT.csv.gz";
    let base_url = "https://datasets.tardis.dev";
    let url = format!("{base_url}/v1/binance-futures/book_snapshot_25/2020/09/01/BTCUSDT.csv.gz");
    ensure_test_data_exists(filename, &url)
}

#[must_use]
pub fn ensure_data_exists_tardis_huobi_quotes() -> PathBuf {
    let filename = "tardis_huobi-dm-swap_quotes_2020-05-01_BTC-USD.csv.gz";
    let base_url = "https://datasets.tardis.dev";
    let url = format!("{base_url}/v1/huobi-dm-swap/quotes/2020/05/01/BTC-USD.csv.gz");
    ensure_test_data_exists(filename, &url)
}

#[must_use]
pub fn ensure_data_exists_tardis_bitmex_trades() -> PathBuf {
    let filename = "tardis_bitmex_trades_2020-03-01_XBTUSD.csv.gz";
    let base_url = "https://datasets.tardis.dev";
    let url = format!("{base_url}/v1/bitmex/trades/2020/03/01/XBTUSD.csv.gz");
    ensure_test_data_exists(filename, &url)
}
