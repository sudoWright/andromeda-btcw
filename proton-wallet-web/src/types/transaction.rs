use std::str::FromStr;

use crate::{
    account::WasmAccount,
    error::{DetailledWasmError, WasmError},
    psbt::WasmPartiallySignedTransaction,
};

use proton_wallet_common::{
    transactions::{DetailledTransaction, DetailledTxOutput, SimpleTransaction, TransactionTime},
    Address, ConfirmationTime, OutPoint, PartiallySignedTransaction, ScriptBuf, Sequence, TxIn,
};
use wasm_bindgen::prelude::*;

use super::{address::WasmAddress, defined::WasmNetwork, derivation_path::WasmDerivationPath};

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct WasmScript(pub Vec<u8>);

impl Into<ScriptBuf> for WasmScript {
    fn into(self) -> ScriptBuf {
        ScriptBuf::from_bytes(self.0)
    }
}

impl Into<ScriptBuf> for &WasmScript {
    fn into(self) -> ScriptBuf {
        ScriptBuf::from_bytes(self.0.clone())
    }
}

impl Into<WasmScript> for ScriptBuf {
    fn into(self) -> WasmScript {
        WasmScript(self.to_bytes())
    }
}

#[wasm_bindgen]
impl WasmScript {
    pub fn to_address(&self, network: WasmNetwork) -> Result<WasmAddress, DetailledWasmError> {
        let script_bug: ScriptBuf = self.into();
        let address = Address::from_script(script_bug.as_script(), network.into()).unwrap();
        Ok(address.into())
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
/// Serialised Outpoint under the form <txid>:<index>
pub struct WasmOutPoint(pub String);

impl TryInto<OutPoint> for WasmOutPoint {
    type Error = WasmError;

    fn try_into(self) -> Result<OutPoint, WasmError> {
        OutPoint::from_str(&self.0).map_err(|_| WasmError::OutpointParsingError)
    }
}

impl Into<WasmOutPoint> for OutPoint {
    fn into(self) -> WasmOutPoint {
        WasmOutPoint(self.to_string())
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct WasmSequence(pub u32);

impl Into<WasmSequence> for Sequence {
    fn into(self) -> WasmSequence {
        WasmSequence(self.0)
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct WasmTxIn {
    pub previous_output: WasmOutPoint,
    pub script_sig: WasmScript,
    pub sequence: WasmSequence,
    // pub witness: WasmWitness, SKIPPY for now, not needed
}

impl Into<WasmTxIn> for TxIn {
    fn into(self) -> WasmTxIn {
        WasmTxIn {
            previous_output: self.previous_output.into(),
            script_sig: self.script_sig.into(),
            sequence: self.sequence.into(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct WasmTxOut {
    pub value: u64,
    pub script_pubkey: WasmScript,
    pub address: WasmAddress,
}

impl Into<WasmTxOut> for DetailledTxOutput {
    fn into(self) -> WasmTxOut {
        WasmTxOut {
            value: self.value,
            script_pubkey: self.script_pubkey.into(),
            address: self.address.into(),
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct WasmDetailledTransaction {
    pub txid: String,
    pub value: i64,
    pub fees: Option<u64>,
    pub time: Option<WasmTransactionTime>,
    pub inputs: Vec<WasmTxIn>,
    pub outputs: Vec<WasmTxOut>,
}

impl Into<WasmDetailledTransaction> for DetailledTransaction {
    fn into(self) -> WasmDetailledTransaction {
        WasmDetailledTransaction {
            txid: self.txid.to_string(),
            value: self.value,
            fees: self.fees,
            time: match self.time {
                Some(time) => Some(time.into()),
                _ => None,
            },
            inputs: self.inputs.into_iter().map(|input| input.into()).collect::<Vec<_>>(),
            outputs: self.outputs.into_iter().map(|output| output.into()).collect::<Vec<_>>(),
        }
    }
}

#[wasm_bindgen]
impl WasmDetailledTransaction {
    pub fn from_psbt(psbt: &WasmPartiallySignedTransaction, account: &WasmAccount) -> Self {
        let psbt: PartiallySignedTransaction = psbt.into();
        let inner = account.get_inner();
        let account = inner.lock().unwrap();
        let wallet = account.get_wallet();

        let tx = DetailledTransaction::from_psbt(&psbt, wallet);
        tx.into()
    }
}

#[wasm_bindgen(getter_with_clone)]
#[derive(Clone)]
pub struct WasmTransactionTime {
    pub confirmed: bool,
    pub confirmation_time: Option<u64>,
    pub last_seen: Option<u64>,
}

impl Into<WasmTransactionTime> for TransactionTime {
    fn into(self) -> WasmTransactionTime {
        match self {
            TransactionTime::Confirmed { confirmation_time } => WasmTransactionTime {
                confirmed: true,
                confirmation_time: Some(confirmation_time),
                last_seen: None,
            },
            TransactionTime::Unconfirmed { last_seen } => WasmTransactionTime {
                confirmed: false,
                confirmation_time: None,
                last_seen: Some(last_seen),
            },
        }
    }
}

impl Into<WasmTransactionTime> for ConfirmationTime {
    fn into(self) -> WasmTransactionTime {
        match self {
            ConfirmationTime::Confirmed { time, .. } => WasmTransactionTime {
                confirmed: true,
                confirmation_time: Some(time),
                last_seen: None,
            },
            ConfirmationTime::Unconfirmed { last_seen } => WasmTransactionTime {
                confirmed: false,
                confirmation_time: None,
                last_seen: Some(last_seen),
            },
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct WasmSimpleTransaction {
    pub txid: String,
    pub value: i64,
    pub fees: Option<u64>,
    pub time: WasmTransactionTime,
    pub account_key: Option<WasmDerivationPath>,
}

impl Into<WasmSimpleTransaction> for SimpleTransaction {
    fn into(self) -> WasmSimpleTransaction {
        WasmSimpleTransaction {
            txid: self.txid.to_string(),
            value: self.value,
            fees: self.fees,
            time: self.time.into(),
            account_key: match self.account_key {
                Some(account_key) => Some(account_key.into()),
                _ => None,
            },
        }
    }
}
