use wasm_bindgen::prelude::*;

use andromeda_bitcoin::BdkBalance;
#[wasm_bindgen]
pub struct WasmBalance {
    /// All coinbase outputs not yet matured
    pub immature: u64,
    /// Unconfirmed UTXOs generated by a wallet tx
    pub trusted_pending: u64,
    /// Unconfirmed UTXOs received from an external wallet
    pub untrusted_pending: u64,
    /// Confirmed and immediately spendable balance
    pub confirmed: u64,
}

impl Into<WasmBalance> for BdkBalance {
    fn into(self) -> WasmBalance {
        WasmBalance {
            immature: self.immature,
            trusted_pending: self.trusted_pending,
            untrusted_pending: self.untrusted_pending,
            confirmed: self.confirmed,
        }
    }
}
