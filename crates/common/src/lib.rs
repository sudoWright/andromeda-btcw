use bdk::bitcoin::{
    bip32::{ChildNumber, DerivationPath},
    Network as BdkNetwork,
};
use serde::{Deserialize, Serialize};

pub const SATOSHI: u64 = 1;
pub const BITCOIN: u64 = 100_000_000 * SATOSHI;
pub const MILLI_BITCOIN: u64 = BITCOIN / 1000;

/// Reimpl of BDK's Network enum to have exhaustive enum
#[derive(Debug, Clone, Copy)]
pub enum Network {
    /// Mainnet Bitcoin.
    Bitcoin,
    /// Bitcoin's testnet network.
    Testnet,
    /// Bitcoin's signet network.
    Signet,
    /// Bitcoin's regtest network.
    Regtest,
}

impl ToString for Network {
    fn to_string(&self) -> String {
        match self {
            Network::Bitcoin => String::from("bitcoin"),
            Network::Testnet => String::from("testnet"),
            Network::Signet => String::from("signet"),
            Network::Regtest => String::from("regtest"),
        }
    }
}

impl From<Network> for BdkNetwork {
    fn from(network: Network) -> Self {
        match network {
            Network::Bitcoin => BdkNetwork::Bitcoin,
            Network::Testnet => BdkNetwork::Testnet,
            Network::Signet => BdkNetwork::Signet,
            Network::Regtest => BdkNetwork::Regtest,
        }
    }
}

impl From<BdkNetwork> for Network {
    fn from(network: BdkNetwork) -> Self {
        match network {
            BdkNetwork::Bitcoin => Network::Bitcoin,
            BdkNetwork::Testnet => Network::Testnet,
            BdkNetwork::Signet => Network::Signet,
            BdkNetwork::Regtest => Network::Regtest,
            _ => panic!("Network {} not supported", network),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Deserialize, Serialize)]
pub enum BitcoinUnit {
    /// 100,000,000 sats
    BTC,
    /// 100,000 sats
    MBTC,
    /// 1 sat
    SAT,
}

pub trait FromParts {
    fn from_parts(purpose: u32, network: Network, account_index: u32) -> Self;
}

impl FromParts for DerivationPath {
    /// Builds a `DerivationPath` from different parts.
    /// Given BIP32: purpose is used as first index, then network to infer
    /// cointype for second index and finally account index for account-level
    /// derivation at third index ```rust
    /// # use std::str::FromStr;
    /// # use bdk::bitcoin::bip32::DerivationPath;
    /// # use andromeda_common::{FromParts, Network};
    /// #
    /// let derivation_path = DerivationPath::from_parts(84, Network::Bitcoin, 0);
    /// assert_eq!(derivation_path, DerivationPath::from_str("m/84'/0'/0'").unwrap());
    /// ```
    fn from_parts(purpose: u32, network: Network, account: u32) -> Self {
        let purpose_level = ChildNumber::from_hardened_idx(purpose).unwrap();

        let network_index = match network {
            Network::Bitcoin => 0,
            _ => 1,
        };
        let cointype_level = ChildNumber::from_hardened_idx(network_index).unwrap();

        let account_level = ChildNumber::from_hardened_idx(account).unwrap();

        DerivationPath::from(vec![purpose_level, cointype_level, account_level])
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum ScriptType {
    /// Legacy scripts : https://bitcoinwiki.org/wiki/pay-to-pubkey-hash
    Legacy = 1,
    /// Nested segwit scrips : https://bitcoinwiki.org/wiki/pay-to-script-hash
    NestedSegwit = 2,
    /// Native segwit scripts : https://bips.dev/173/
    NativeSegwit = 3,
    /// Taproot scripts : https://bips.dev/341/
    Taproot = 4,
}

impl Into<u8> for ScriptType {
    fn into(self) -> u8 {
        match self {
            ScriptType::Legacy => 1u8,
            ScriptType::NestedSegwit => 2u8,
            ScriptType::NativeSegwit => 3u8,
            ScriptType::Taproot => 4u8,
        }
    }
}

impl From<ScriptType> for ChildNumber {
    /// Returns default purpose derivation index (level 1) for each script type
    /// ```
    /// # use bdk::bitcoin::bip32::ChildNumber;
    /// # use andromeda_common::ScriptType;
    /// #
    /// let purpose: ChildNumber = ScriptType::NestedSegwit.into();
    /// assert_eq!(purpose, ChildNumber::Hardened { index: 49 });
    /// ```
    fn from(value: ScriptType) -> Self {
        match value {
            ScriptType::Legacy => ChildNumber::Hardened { index: 44 },
            ScriptType::NestedSegwit => ChildNumber::Hardened { index: 49 },
            ScriptType::NativeSegwit => ChildNumber::Hardened { index: 84 },
            ScriptType::Taproot => ChildNumber::Hardened { index: 86 },
        }
    }
}
