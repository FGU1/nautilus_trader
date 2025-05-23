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

use std::fmt::Display;

use derive_builder::Builder;
use nautilus_core::{UUID4, UnixNanos};
use nautilus_model::identifiers::{ClientId, InstrumentId, StrategyId, TraderId};
use serde::{Deserialize, Serialize};

use super::cancel::CancelOrder;

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize, Builder)]
#[builder(default)]
#[serde(tag = "type")]
pub struct BatchCancelOrders {
    pub trader_id: TraderId,
    pub client_id: ClientId,
    pub strategy_id: StrategyId,
    pub instrument_id: InstrumentId,
    pub cancels: Vec<CancelOrder>,
    pub command_id: UUID4,
    pub ts_init: UnixNanos,
}

impl BatchCancelOrders {
    /// Creates a new [`BatchCancelOrders`] instance.
    ///
    /// # Errors
    ///
    /// Returns an error if parameters are invalid.
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        trader_id: TraderId,
        client_id: ClientId,
        strategy_id: StrategyId,
        instrument_id: InstrumentId,
        cancels: Vec<CancelOrder>,
        command_id: UUID4,
        ts_init: UnixNanos,
    ) -> anyhow::Result<Self> {
        Ok(Self {
            trader_id,
            client_id,
            strategy_id,
            instrument_id,
            cancels,
            command_id,
            ts_init,
        })
    }
}

impl Display for BatchCancelOrders {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "BatchCancelOrders(instrument_id={}, cancels=TBD)",
            self.instrument_id,
        )
    }
}

////////////////////////////////////////////////////////////////////////////////
// Tests
////////////////////////////////////////////////////////////////////////////////
#[cfg(test)]
mod tests {}
