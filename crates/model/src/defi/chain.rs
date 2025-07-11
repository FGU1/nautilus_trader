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

//! Basic structures for representing on-chain blocks and transactions in DeFi integrations.

use std::{
    fmt::{Display, Formatter},
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, EnumString};

/// Represents different blockchain networks.
#[derive(
    Debug,
    Clone,
    Copy,
    Hash,
    PartialOrd,
    PartialEq,
    Ord,
    Eq,
    Display,
    EnumIter,
    EnumString,
    Serialize,
    Deserialize,
)]
#[non_exhaustive]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "nautilus_trader.core.nautilus_pyo3.model")
)]
pub enum Blockchain {
    Abstract,
    Arbitrum,
    ArbitrumNova,
    ArbitrumSepolia,
    Aurora,
    Avalanche,
    Base,
    BaseSepolia,
    Berachain,
    BerachainBartio,
    Blast,
    BlastSepolia,
    Boba,
    Bsc,
    BscTestnet,
    Celo,
    Chiliz,
    CitreaTestnet,
    Curtis,
    Cyber,
    Darwinia,
    Ethereum,
    Fantom,
    Flare,
    Fraxtal,
    Fuji,
    GaladrielDevnet,
    Gnosis,
    GnosisChiado,
    GnosisTraces,
    HarmonyShard0,
    Holesky,
    HoleskyTokenTest,
    Hyperliquid,
    HyperliquidTemp,
    Ink,
    InternalTestChain,
    Kroma,
    Linea,
    Lisk,
    Lukso,
    LuksoTestnet,
    Manta,
    Mantle,
    MegaethTestnet,
    Merlin,
    Metall2,
    Metis,
    MevCommit,
    Mode,
    MonadTestnet,
    MonadTestnetBackup,
    MoonbaseAlpha,
    Moonbeam,
    Morph,
    MorphHolesky,
    Opbnb,
    Optimism,
    OptimismSepolia,
    PharosDevnet,
    Polygon,
    PolygonAmoy,
    PolygonZkEvm,
    Rootstock,
    Saakuru,
    Scroll,
    Sepolia,
    ShimmerEvm,
    Soneium,
    Sophon,
    SophonTestnet,
    Superseed,
    Unichain,
    UnichainSepolia,
    Xdc,
    XdcTestnet,
    Zeta,
    Zircuit,
    ZKsync,
    Zora,
}

/// Defines a blockchain with its unique identifiers and connection details for network interaction.
#[cfg_attr(feature = "python", pyo3::pyclass(module = "nautilus_pyo3.model"))]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chain {
    /// The blockchain network type.
    pub name: Blockchain,
    /// The unique identifier for this blockchain.
    pub chain_id: u32,
    /// URL endpoint for HyperSync connection.
    pub hypersync_url: String,
    /// URL endpoint for the default RPC connection.
    pub rpc_url: Option<String>,
    /// The number of decimals for the native currency.
    pub native_currency_decimals: u8,
}

/// A thread-safe shared pointer to a `Chain`, enabling efficient reuse across multiple components.
pub type SharedChain = Arc<Chain>;

impl Chain {
    /// Creates a new [`Chain`] instance with the specified blockchain and chain ID.
    pub fn new(name: Blockchain, chain_id: u32) -> Self {
        Self {
            chain_id,
            name,
            hypersync_url: format!("https://{chain_id}.hypersync.xyz"),
            rpc_url: None,
            native_currency_decimals: 18, // Default to 18 for EVM chains
        }
    }

    /// Sets the RPC URL endpoint.
    pub fn set_rpc_url(&mut self, rpc: String) {
        self.rpc_url = Some(rpc);
    }

    /// Returns a reference to the `Chain` corresponding to the given `chain_id`, or `None` if it is not found.
    pub fn from_chain_id(chain_id: u32) -> Option<&'static Chain> {
        match chain_id {
            2741 => Some(&chains::ABSTRACT),
            42161 => Some(&chains::ARBITRUM),
            42170 => Some(&chains::ARBITRUM_NOVA),
            421614 => Some(&chains::ARBITRUM_SEPOLIA),
            1313161554 => Some(&chains::AURORA),
            43114 => Some(&chains::AVALANCHE),
            8453 => Some(&chains::BASE),
            84532 => Some(&chains::BASE_SEPOLIA),
            80094 => Some(&chains::BERACHAIN),
            80085 => Some(&chains::BERACHAIN_BARTIO),
            81457 => Some(&chains::BLAST),
            168587773 => Some(&chains::BLAST_SEPOLIA),
            288 => Some(&chains::BOBA),
            56 => Some(&chains::BSC),
            97 => Some(&chains::BSC_TESTNET),
            42220 => Some(&chains::CELO),
            8888 => Some(&chains::CHILIZ),
            3333 => Some(&chains::CITREA_TESTNET),
            33111 => Some(&chains::CURTIS),
            7560 => Some(&chains::CYBER),
            46 => Some(&chains::DARWINIA),
            1 => Some(&chains::ETHEREUM),
            250 => Some(&chains::FANTOM),
            14 => Some(&chains::FLARE),
            252 => Some(&chains::FRAXTAL),
            43113 => Some(&chains::FUJI),
            696969 => Some(&chains::GALADRIEL_DEVNET),
            100 => Some(&chains::GNOSIS),
            10200 => Some(&chains::GNOSIS_CHIADO),
            10300 => Some(&chains::GNOSIS_TRACES),
            1666600000 => Some(&chains::HARMONY_SHARD_0),
            17000 => Some(&chains::HOLESKY),
            17001 => Some(&chains::HOLESKY_TOKEN_TEST),
            7979 => Some(&chains::HYPERLIQUID),
            7978 => Some(&chains::HYPERLIQUID_TEMP),
            222 => Some(&chains::INK),
            13337 => Some(&chains::INTERNAL_TEST_CHAIN),
            255 => Some(&chains::KROMA),
            59144 => Some(&chains::LINEA),
            501 => Some(&chains::LISK),
            42 => Some(&chains::LUKSO),
            4201 => Some(&chains::LUKSO_TESTNET),
            169 => Some(&chains::MANTA),
            5000 => Some(&chains::MANTLE),
            777 => Some(&chains::MEGAETH_TESTNET),
            4200 => Some(&chains::MERLIN),
            90 => Some(&chains::METALL2),
            1088 => Some(&chains::METIS),
            11 => Some(&chains::MEV_COMMIT),
            34443 => Some(&chains::MODE),
            2323 => Some(&chains::MONAD_TESTNET),
            2358 => Some(&chains::MONAD_TESTNET_BACKUP),
            1287 => Some(&chains::MOONBASE_ALPHA),
            1284 => Some(&chains::MOONBEAM),
            2710 => Some(&chains::MORPH),
            2710111 => Some(&chains::MORPH_HOLESKY),
            204 => Some(&chains::OPBNB),
            10 => Some(&chains::OPTIMISM),
            11155420 => Some(&chains::OPTIMISM_SEPOLIA),
            1337 => Some(&chains::PHAROS_DEVNET),
            137 => Some(&chains::POLYGON),
            80002 => Some(&chains::POLYGON_AMOY),
            1101 => Some(&chains::POLYGON_ZKEVM),
            30 => Some(&chains::ROOTSTOCK),
            1204 => Some(&chains::SAAKURU),
            534352 => Some(&chains::SCROLL),
            11155111 => Some(&chains::SEPOLIA),
            148 => Some(&chains::SHIMMER_EVM),
            109 => Some(&chains::SONEIUM),
            138 => Some(&chains::SOPHON),
            139 => Some(&chains::SOPHON_TESTNET),
            10001 => Some(&chains::SUPERSEED),
            9999 => Some(&chains::UNICHAIN),
            9997 => Some(&chains::UNICHAIN_SEPOLIA),
            50 => Some(&chains::XDC),
            51 => Some(&chains::XDC_TESTNET),
            7000 => Some(&chains::ZETA),
            78600 => Some(&chains::ZIRCUIT),
            324 => Some(&chains::ZKSYNC),
            7777777 => Some(&chains::ZORA),
            _ => None,
        }
    }
}

impl Display for Chain {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Chain(name={}, id={})", self.name, self.chain_id)
    }
}

// Define a module to contain all the chain definitions.
pub mod chains {
    use std::sync::LazyLock;

    use crate::defi::chain::{Blockchain, Chain};

    pub static ABSTRACT: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Abstract, 2741));
    pub static ARBITRUM: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::Arbitrum, 42161));
    pub static ARBITRUM_NOVA: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::ArbitrumNova, 42170));
    pub static ARBITRUM_SEPOLIA: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::ArbitrumSepolia, 421614));
    pub static AURORA: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::Aurora, 1313161554));
    pub static AVALANCHE: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::Avalanche, 43114));
    pub static BASE: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Base, 8453));
    pub static BASE_SEPOLIA: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::BaseSepolia, 84532));
    pub static BERACHAIN: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::Berachain, 80094));
    pub static BERACHAIN_BARTIO: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::BerachainBartio, 80085));
    pub static BLAST: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Blast, 81457));
    pub static BLAST_SEPOLIA: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::BlastSepolia, 168587773));
    pub static BOBA: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Boba, 288));
    pub static BSC: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Bsc, 56));
    pub static BSC_TESTNET: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::BscTestnet, 97));
    pub static CELO: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Celo, 42220));
    pub static CHILIZ: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Chiliz, 8888));
    pub static CITREA_TESTNET: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::CitreaTestnet, 3333));
    pub static CURTIS: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Curtis, 33111));
    pub static CYBER: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Cyber, 7560));
    pub static DARWINIA: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Darwinia, 46));
    pub static ETHEREUM: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Ethereum, 1));
    pub static FANTOM: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Fantom, 250));
    pub static FLARE: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Flare, 14));
    pub static FRAXTAL: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Fraxtal, 252));
    pub static FUJI: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Fuji, 43113));
    pub static GALADRIEL_DEVNET: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::GaladrielDevnet, 696969));
    pub static GNOSIS: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Gnosis, 100));
    pub static GNOSIS_CHIADO: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::GnosisChiado, 10200));
    // Chain ID 10300 is reserved for the public *Gnosis Traces* test-network. The value was
    // previously set to 100 (Mainnet) which caused `Chain::from_chain_id(10300)` to return a
    // `Chain` whose `chain_id` field did not match the requested ID. This led to confusing log
    // output and could break caching keyed by the numeric identifier. We therefore align the
    // static definition with the mapping used in `from_chain_id` (10300).
    pub static GNOSIS_TRACES: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::GnosisTraces, 10300));
    pub static HARMONY_SHARD_0: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::HarmonyShard0, 1666600000));
    pub static HOLESKY: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Holesky, 17000));
    // The Holesky *token test* network uses a dedicated chain-ID (17001) distinct from the main
    // Holesky devnet (17000). Align this constant with the value returned from `from_chain_id`.
    pub static HOLESKY_TOKEN_TEST: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::HoleskyTokenTest, 17001));
    // Hyperliquid main & temp test networks live on low numeric identifiers (7979 / 7978).
    // Using the correct small IDs avoids overflow issues in certain front-ends that assume
    // EVM-style 32-bit chain IDs.
    pub static HYPERLIQUID: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::Hyperliquid, 7979));
    pub static HYPERLIQUID_TEMP: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::HyperliquidTemp, 7978));
    // Align with mapping – 222 is the well–known chain-ID for the `Ink` network.
    pub static INK: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Ink, 222));
    // Use the `foundry`-style development chain-ID 13337 to match the lookup table above.
    pub static INTERNAL_TEST_CHAIN: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::InternalTestChain, 13337));
    pub static KROMA: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Kroma, 255));
    pub static LINEA: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Linea, 59144));
    pub static LISK: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Lisk, 501));
    pub static LUKSO: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Lukso, 42));
    pub static LUKSO_TESTNET: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::LuksoTestnet, 4201));
    pub static MANTA: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Manta, 169));
    pub static MANTLE: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Mantle, 5000));
    pub static MEGAETH_TESTNET: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::MegaethTestnet, 777));
    pub static MERLIN: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Merlin, 4200));
    pub static METALL2: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Metall2, 90));
    pub static METIS: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Metis, 1088));
    pub static MEV_COMMIT: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::MevCommit, 11));
    pub static MODE: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Mode, 34443));
    pub static MONAD_TESTNET: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::MonadTestnet, 2323));
    pub static MONAD_TESTNET_BACKUP: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::MonadTestnetBackup, 2358));
    pub static MOONBASE_ALPHA: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::MoonbaseAlpha, 1287));
    pub static MOONBEAM: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Moonbeam, 1284));
    pub static MORPH: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Morph, 2710));
    pub static MORPH_HOLESKY: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::MorphHolesky, 2710111));
    pub static OPBNB: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Opbnb, 204));
    pub static OPTIMISM: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Optimism, 10));
    pub static OPTIMISM_SEPOLIA: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::OptimismSepolia, 11155420));
    pub static PHAROS_DEVNET: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::PharosDevnet, 1337));
    pub static POLYGON: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Polygon, 137));
    pub static POLYGON_AMOY: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::PolygonAmoy, 80002));
    pub static POLYGON_ZKEVM: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::PolygonZkEvm, 1101));
    pub static ROOTSTOCK: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Rootstock, 30));
    pub static SAAKURU: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Saakuru, 1204));
    pub static SCROLL: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Scroll, 534352));
    pub static SEPOLIA: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::Sepolia, 11155111));
    pub static SHIMMER_EVM: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::ShimmerEvm, 148));
    pub static SONEIUM: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Soneium, 109));
    pub static SOPHON: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Sophon, 138));
    pub static SOPHON_TESTNET: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::SophonTestnet, 139));
    pub static SUPERSEED: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::Superseed, 10001));
    pub static UNICHAIN: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Unichain, 9999));
    pub static UNICHAIN_SEPOLIA: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::UnichainSepolia, 9997));
    pub static XDC: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Xdc, 50));
    pub static XDC_TESTNET: LazyLock<Chain> =
        LazyLock::new(|| Chain::new(Blockchain::XdcTestnet, 51));
    pub static ZETA: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Zeta, 7000));
    pub static ZIRCUIT: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Zircuit, 78600));
    pub static ZKSYNC: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::ZKsync, 324));
    pub static ZORA: LazyLock<Chain> = LazyLock::new(|| Chain::new(Blockchain::Zora, 7777777));
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_ethereum_chain() {
        let eth_chain = chains::ETHEREUM.clone();
        assert_eq!(eth_chain.to_string(), "Chain(name=Ethereum, id=1)");
        assert_eq!(eth_chain.name, Blockchain::Ethereum);
        assert_eq!(eth_chain.chain_id, 1);
        assert_eq!(eth_chain.hypersync_url.as_str(), "https://1.hypersync.xyz")
    }

    #[rstest]
    fn test_arbitrum_chain() {
        let arbitrum_chain = chains::ARBITRUM.clone();
        assert_eq!(arbitrum_chain.to_string(), "Chain(name=Arbitrum, id=42161)");
        assert_eq!(arbitrum_chain.name, Blockchain::Arbitrum);
        assert_eq!(arbitrum_chain.chain_id, 42161);
        assert_eq!(
            arbitrum_chain.hypersync_url.as_str(),
            "https://42161.hypersync.xyz"
        );
    }

    #[rstest]
    fn test_chain_constructor() {
        let chain = Chain::new(Blockchain::Polygon, 137);

        assert_eq!(chain.name, Blockchain::Polygon);
        assert_eq!(chain.chain_id, 137);
        assert_eq!(chain.hypersync_url, "https://137.hypersync.xyz");
        assert!(chain.rpc_url.is_none());
        assert_eq!(chain.native_currency_decimals, 18);
    }

    #[rstest]
    fn test_chain_set_rpc_url() {
        let mut chain = Chain::new(Blockchain::Ethereum, 1);
        assert!(chain.rpc_url.is_none());

        let rpc_url = "https://mainnet.infura.io/v3/YOUR-PROJECT-ID".to_string();
        chain.set_rpc_url(rpc_url.clone());

        assert_eq!(chain.rpc_url, Some(rpc_url));
    }

    #[rstest]
    fn test_chain_from_chain_id_valid() {
        // Test some known chain IDs
        assert!(Chain::from_chain_id(1).is_some()); // Ethereum
        assert!(Chain::from_chain_id(137).is_some()); // Polygon
        assert!(Chain::from_chain_id(42161).is_some()); // Arbitrum
        assert!(Chain::from_chain_id(8453).is_some()); // Base

        // Verify specific chain
        let eth_chain = Chain::from_chain_id(1).unwrap();
        assert_eq!(eth_chain.name, Blockchain::Ethereum);
        assert_eq!(eth_chain.chain_id, 1);
    }

    #[rstest]
    fn test_chain_from_chain_id_invalid() {
        // Test unknown chain ID
        assert!(Chain::from_chain_id(999999).is_none());
        assert!(Chain::from_chain_id(0).is_none());
    }
}
