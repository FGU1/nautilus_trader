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

use std::sync::LazyLock;

use nautilus_model::defi::{
    chain::chains,
    dex::{AmmType, Dex},
};

use crate::exchanges::extended::DexExtended;

/// Uniswap V2 DEX on Ethereum.
pub static UNISWAP_V2: LazyLock<DexExtended> = LazyLock::new(|| {
    let dex = Dex::new(
        chains::ETHEREUM.clone(),
        "Uniswap V2",
        "0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f",
        AmmType::CPAMM,
        "PoolCreated(address,address,address,uint256)",
        "",
        "",
        "",
    );
    DexExtended::new(dex)
});
