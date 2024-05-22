use std::sync::Arc;

use serde::Deserialize;

use super::BASE_WALLET_API_V1;
use crate::{
    core::{ProtonResponseExt, ToProtonRequest},
    error::Error,
    transaction::ApiTransactionStatus,
    ProtonWalletApiClient,
};

pub struct AddressClient {
    api_client: Arc<ProtonWalletApiClient>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct AddressBalance {
    pub Address: String,
    pub ChainFundedBitcoin: u64,
    pub ChainSpentBitcoin: u64,
    pub MempoolFundedBitcoin: u64,
    pub MempoolSpentBitcoin: u64,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct GetAddressBalanceResponseBody {
    pub Code: u16,
    pub Balance: AddressBalance,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct ApiVout {
    pub ScriptPubKey: String,
    pub ScriptPubKeyAsm: String,
    pub ScriptPubKeyType: String,
    pub ScriptPubKeyAddress: String,
    pub Value: u64,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct ApiVin {
    pub TransactionId: String,
    pub Vout: u32,
    pub Prevout: ApiVout,
    pub ScriptSig: String,
    pub ScriptSigAsm: String,
    pub Witness: Vec<String>,
    pub InnerWitnessScriptAsm: Option<String>,
    pub IsCoinbase: u8,
    pub Sequence: u32,
    pub InnerRedeemScriptAsm: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct ApiTx {
    pub TransactionId: String,
    pub Version: i32,
    pub Locktime: u32,
    pub Vin: Option<Vec<ApiVin>>,
    pub Vout: Option<Vec<ApiVout>>,
    pub Size: u32,
    pub Weight: u32,
    pub Fee: u64,
    pub TransactionStatus: ApiTransactionStatus,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct GetScriptHashTransactionsResponseBody {
    pub Code: u16,
    pub Transactions: Vec<ApiTx>,
}

#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct GetScriptHashTransactionsAtTransactionIdResponseBody {
    pub Code: u16,
    pub Transactions: Vec<ApiTx>,
}

impl AddressClient {
    pub fn new(api_client: Arc<ProtonWalletApiClient>) -> Self {
        Self { api_client }
    }

    /// Get recent block summaries, starting at tip or height if provided
    pub async fn get_address_balance(&self, address: String) -> Result<AddressBalance, Error> {
        let request = self
            .api_client
            .build_full_url(BASE_WALLET_API_V1, format!("addresses/{}/balance", address))
            .to_get_request();

        let response = self.api_client.send(request).await?;
        let parsed = response.parse_response::<GetAddressBalanceResponseBody>()?;

        Ok(parsed.Balance)
    }

    /// Get a [`BlockHeader`] given a particular block hash.
    pub async fn get_scripthash_transactions(&self, script_hash: String) -> Result<Vec<ApiTx>, Error> {
        let request = self
            .api_client
            .build_full_url(
                BASE_WALLET_API_V1,
                format!("addresses/scripthash/{}/transactions", script_hash),
            )
            .to_get_request();

        let response = self.api_client.send(request).await?;
        let parsed = response.parse_response::<GetScriptHashTransactionsResponseBody>()?;

        Ok(parsed.Transactions)
    }

    /// Get a [`BlockHeader`] given a particular block hash.
    pub async fn get_scripthash_transactions_at_transaction_id(
        &self,
        script_hash: String,
        transaction_id: String,
    ) -> Result<Vec<ApiTx>, Error> {
        let request = self
            .api_client
            .build_full_url(
                BASE_WALLET_API_V1,
                format!("addresses/scripthash/{}/transactions/{}", script_hash, transaction_id),
            )
            .to_get_request();

        let response = self.api_client.send(request).await?;
        let parsed = response.parse_response::<GetScriptHashTransactionsResponseBody>()?;

        Ok(parsed.Transactions)
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use bitcoin::{
        hashes::{sha256, Hash},
        Address,
    };

    use super::AddressClient;
    use crate::tests::utils::common_api_client;

    #[tokio::test]
    #[ignore]
    async fn should_get_address_balance() {
        let api_client = common_api_client().await;
        let client = AddressClient::new(api_client);

        let address = client
            .get_address_balance(String::from("tb1q886jdswcmtn5u9memdlaz0lymua637a9aufqq6"))
            .await;

        println!("request done: {:?}", address);
    }

    #[tokio::test]
    #[ignore]
    async fn should_get_scripthash_transactions() {
        let api_client = common_api_client().await;
        let client = AddressClient::new(api_client);

        let scripthash = sha256::Hash::hash(
            Address::from_str("tb1q886jdswcmtn5u9memdlaz0lymua637a9aufqq6")
                .unwrap()
                .assume_checked()
                .script_pubkey()
                .as_bytes(),
        );

        let blocks = client.get_scripthash_transactions(scripthash.to_string()).await;

        println!("request done: {:?}", blocks);
    }

    #[tokio::test]
    #[ignore]
    async fn should_get_scripthash_transactions_at_transaction_id() {
        let api_client = common_api_client().await;
        let client = AddressClient::new(api_client);

        let scripthash = sha256::Hash::hash(
            Address::from_str("tb1q886jdswcmtn5u9memdlaz0lymua637a9aufqq6")
                .unwrap()
                .assume_checked()
                .script_pubkey()
                .as_bytes(),
        );

        let blocks = client
            .get_scripthash_transactions_at_transaction_id(
                scripthash.to_string(),
                String::from("ed33ee58f14efbd8a2d9fb6e30bde660c07512dbc7b4d80e00975d3e3d16a302"),
            )
            .await;

        println!("request done: {:?}", blocks);
    }
}
