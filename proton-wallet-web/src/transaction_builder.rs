use proton_wallet_common::{
    transaction_builder::{CoinSelection, TmpRecipient, TxBuilder},
    ChangeSpendPolicy, OutPoint,
};
use wasm_bindgen::prelude::*;
use web_sys::console::log_2;

use crate::{
    account::WasmAccount,
    error::{DetailledWasmError, WasmError},
    psbt::WasmPartiallySignedTransaction,
    storage::OnchainStorage,
    types::{defined::WasmNetwork, locktime::WasmLockTime, transaction::WasmOutPoint},
};

#[wasm_bindgen]
pub struct WasmTxBuilder {
    inner: TxBuilder<OnchainStorage>,
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum WasmCoinSelection {
    BranchAndBound,
    LargestFirst,
    OldestFirst,
    Manual,
}

impl Into<CoinSelection> for WasmCoinSelection {
    fn into(self) -> CoinSelection {
        match self {
            WasmCoinSelection::BranchAndBound => CoinSelection::BranchAndBound,
            WasmCoinSelection::LargestFirst => CoinSelection::LargestFirst,
            WasmCoinSelection::OldestFirst => CoinSelection::OldestFirst,
            WasmCoinSelection::Manual => CoinSelection::Manual,
        }
    }
}

impl Into<WasmCoinSelection> for CoinSelection {
    fn into(self) -> WasmCoinSelection {
        match self {
            CoinSelection::BranchAndBound => WasmCoinSelection::BranchAndBound,
            CoinSelection::LargestFirst => WasmCoinSelection::LargestFirst,
            CoinSelection::OldestFirst => WasmCoinSelection::OldestFirst,
            CoinSelection::Manual => WasmCoinSelection::Manual,
        }
    }
}

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum WasmChangeSpendPolicy {
    ChangeAllowed,
    OnlyChange,
    ChangeForbidden,
}

impl Into<ChangeSpendPolicy> for WasmChangeSpendPolicy {
    fn into(self) -> ChangeSpendPolicy {
        match self {
            WasmChangeSpendPolicy::ChangeAllowed => ChangeSpendPolicy::ChangeAllowed,
            WasmChangeSpendPolicy::OnlyChange => ChangeSpendPolicy::OnlyChange,
            WasmChangeSpendPolicy::ChangeForbidden => ChangeSpendPolicy::ChangeForbidden,
        }
    }
}

impl Into<WasmChangeSpendPolicy> for ChangeSpendPolicy {
    fn into(self) -> WasmChangeSpendPolicy {
        match self {
            ChangeSpendPolicy::ChangeAllowed => WasmChangeSpendPolicy::ChangeAllowed,
            ChangeSpendPolicy::OnlyChange => WasmChangeSpendPolicy::OnlyChange,
            ChangeSpendPolicy::ChangeForbidden => WasmChangeSpendPolicy::ChangeForbidden,
        }
    }
}

#[wasm_bindgen(getter_with_clone)]
pub struct WasmRecipient(pub String, pub String, pub u64);

#[wasm_bindgen]
impl WasmTxBuilder {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmTxBuilder {
        WasmTxBuilder {
            inner: TxBuilder::new(),
        }
    }

    #[wasm_bindgen]
    pub fn set_account(&self, account: &WasmAccount) -> Self {
        let inner = self.inner.set_account(account.get_inner());

        log_2(
            &"account_set".into(),
            &account.get_inner().lock().unwrap().get_derivation_path().to_string().into(),
        );
        WasmTxBuilder { inner }
    }

    #[wasm_bindgen]
    pub fn add_recipient(&self) -> WasmTxBuilder {
        let inner = self.inner.add_recipient();
        WasmTxBuilder { inner }
    }

    #[wasm_bindgen]
    pub fn remove_recipient(&self, index: usize) -> WasmTxBuilder {
        let inner = self.inner.remove_recipient(index);
        WasmTxBuilder { inner }
    }

    #[wasm_bindgen]
    pub fn update_recipient(
        &self,
        index: usize,
        address_str: Option<String>,
        amount: Option<u64>,
    ) -> Result<WasmTxBuilder, WasmError> {
        let inner = self.inner.update_recipient(index, (address_str, amount));
        Ok(WasmTxBuilder { inner })
    }

    pub fn get_recipients(&self) -> Vec<WasmRecipient> {
        let recipients = self
            .inner
            .recipients
            .clone()
            .into_iter()
            .map(|recipient| {
                let TmpRecipient(uuid, address, amount) = recipient;
                let wasm_recipient: WasmRecipient = WasmRecipient(uuid, address, amount);
                wasm_recipient
            })
            .collect();

        recipients
    }

    /**
     * UTXOs
     */

    #[wasm_bindgen]
    pub fn add_utxo_to_spend(&self, outpoint: WasmOutPoint) -> Result<WasmTxBuilder, WasmError> {
        let serialised: OutPoint = outpoint.try_into()?;
        let inner = self.inner.add_utxo_to_spend(&serialised);

        Ok(WasmTxBuilder { inner })
    }

    #[wasm_bindgen]
    pub fn remove_utxo_to_spend(&self, outpoint: WasmOutPoint) -> Result<WasmTxBuilder, WasmError> {
        let serialised: OutPoint = outpoint.try_into()?;
        let inner = self.inner.remove_utxo_to_spend(&serialised);

        Ok(WasmTxBuilder { inner })
    }

    #[wasm_bindgen]
    pub fn clear_utxos_to_spend(&self) -> WasmTxBuilder {
        let inner = self.inner.clear_utxos_to_spend();
        WasmTxBuilder { inner }
    }

    #[wasm_bindgen]
    pub fn get_utxos_to_spend(&self) -> Vec<WasmOutPoint> {
        self.inner
            .utxos_to_spend
            .clone()
            .into_iter()
            .map(|outpoint| {
                let utxo: WasmOutPoint = outpoint.into();
                utxo
            })
            .collect()
    }

    /**
     * Coin selection enforcement
     */

    #[wasm_bindgen]
    pub fn set_coin_selection(&self, coin_selection: WasmCoinSelection) -> Self {
        let inner = self.inner.set_coin_selection(coin_selection.into());
        WasmTxBuilder { inner }
    }

    #[wasm_bindgen]
    pub fn get_coin_selection(&self) -> WasmCoinSelection {
        self.inner.coin_selection.clone().into()
    }

    /**
     * RBF
     */

    #[wasm_bindgen]
    pub fn enable_rbf(&self) -> WasmTxBuilder {
        let inner = self.inner.enable_rbf();
        WasmTxBuilder { inner }
    }

    #[wasm_bindgen]
    pub fn disable_rbf(&self) -> WasmTxBuilder {
        let inner = self.inner.disable_rbf();
        WasmTxBuilder { inner }
    }

    #[wasm_bindgen]
    pub fn get_rbf_enabled(&self) -> bool {
        self.inner.rbf_enabled
    }

    /**
     * Change policy
     */

    #[wasm_bindgen]
    pub fn set_change_policy(&self, change_policy: WasmChangeSpendPolicy) -> Self {
        let inner = self.inner.set_change_policy(change_policy.into());
        WasmTxBuilder { inner }
    }

    #[wasm_bindgen]
    pub fn get_change_policy(&self) -> WasmChangeSpendPolicy {
        self.inner.change_policy.into()
    }

    /**
     * Fees
     */

    #[wasm_bindgen]
    pub fn set_fee_rate(&self, sat_per_vb: f32) -> WasmTxBuilder {
        let inner = self.inner.set_fee_rate(sat_per_vb);
        WasmTxBuilder { inner }
    }

    #[wasm_bindgen]
    pub fn get_fee_rate(&self) -> Option<f32> {
        if let Some(fee_rate) = self.inner.fee_rate {
            Some(fee_rate.as_sat_per_vb())
        } else {
            None
        }
    }

    /**
     * Locktime
     */

    #[wasm_bindgen]
    pub fn add_locktime(&self, locktime: WasmLockTime) -> Self {
        let inner = self.inner.add_locktime(locktime.into());
        WasmTxBuilder { inner }
    }

    #[wasm_bindgen]
    pub fn remove_locktime(&self) -> Self {
        let inner = self.inner.remove_locktime();
        WasmTxBuilder { inner }
    }

    #[wasm_bindgen]
    pub fn get_locktime(&self) -> Option<WasmLockTime> {
        match self.inner.locktime {
            Some(locktime) => Some(locktime.into()),
            _ => None,
        }
    }

    /**
     * Final
     */

    #[wasm_bindgen]
    pub fn create_pbst(&self, network: WasmNetwork) -> Result<WasmPartiallySignedTransaction, DetailledWasmError> {
        let psbt = self
            .inner
            .create_pbst_with_coin_selection(false)
            .map_err(|e| e.into())?;

        Ok(WasmPartiallySignedTransaction::from_psbt(&psbt, network.into()))
    }
}
