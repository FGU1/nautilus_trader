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

use nautilus_model::{
    data::{Bar, QuoteTick, TradeTick},
    enums::PriceType,
};
use pyo3::prelude::*;

use crate::{
    average::MovingAverageType,
    indicator::{Indicator, MovingAverage},
    momentum::macd::MovingAverageConvergenceDivergence,
};

#[pymethods]
impl MovingAverageConvergenceDivergence {
    #[new]
    #[pyo3(signature = (fast_period, slow_period, ma_type=None, price_type=None))]
    #[must_use]
    pub fn py_new(
        fast_period: usize,
        slow_period: usize,
        ma_type: Option<MovingAverageType>,
        price_type: Option<PriceType>,
    ) -> Self {
        Self::new(fast_period, slow_period, ma_type, price_type)
    }

    fn __repr__(&self) -> String {
        format!(
            "MovingAverageConvergenceDivergence({},{},{:?})",
            self.fast_period, self.slow_period, self.price_type
        )
    }

    #[getter]
    #[pyo3(name = "name")]
    fn py_name(&self) -> String {
        self.name()
    }

    #[getter]
    #[pyo3(name = "fast_period")]
    const fn py_fast_period(&self) -> usize {
        self.fast_period
    }

    #[getter]
    #[pyo3(name = "slow_period")]
    const fn py_slow_period(&self) -> usize {
        self.slow_period
    }

    #[getter]
    #[pyo3(name = "count")]
    fn py_count(&self) -> usize {
        self.count()
    }

    #[getter]
    #[pyo3(name = "has_inputs")]
    fn py_has_inputs(&self) -> bool {
        self.has_inputs()
    }

    #[getter]
    #[pyo3(name = "initialized")]
    const fn py_initialized(&self) -> bool {
        self.initialized
    }

    #[getter]
    #[pyo3(name = "value")]
    const fn py_value(&self) -> f64 {
        self.value
    }

    #[pyo3(name = "handle_quote_tick")]
    fn py_handle_quote_tick(&mut self, quote: &QuoteTick) {
        self.py_update_raw(quote.extract_price(self.price_type).into());
    }

    #[pyo3(name = "handle_trade_tick")]
    fn py_handle_trade_tick(&mut self, trade: &TradeTick) {
        self.update_raw((&trade.price).into());
    }

    #[pyo3(name = "handle_bar")]
    fn py_handle_bar(&mut self, bar: &Bar) {
        self.update_raw((&bar.close).into());
    }

    #[pyo3(name = "reset")]
    fn py_reset(&mut self) {
        self.reset();
    }

    #[pyo3(name = "update_raw")]
    fn py_update_raw(&mut self, close: f64) {
        self.update_raw(close);
    }
}
