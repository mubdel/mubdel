use std::str::FromStr;

use crate::{CryptoCurrency, Wallet};

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, system_transaction};

pub struct Solana {}

impl CryptoCurrency for Solana {
    async fn create_wallet() -> Wallet {
        let keypair = Keypair::new();

        Wallet {
            public_key: keypair.pubkey().to_string(),
            private_key: keypair.to_base58_string(),
        }
    }

    async fn transfer(&self, sender: String, recipient: String, amount: u64) -> Result<()> {
        let rpc_client = RpcClient::new("https://api.testnet.solana.com");
        let to = Pubkey::from_str(&recipient)?;
        let from = Keypair::from_base58_string(&sender);
        let latest_blockhash = rpc_client.get_latest_blockhash()?;
        let tx = system_transaction::transfer(&from, &to, amount, latest_blockhash);
        rpc_client.send_and_confirm_transaction(&tx)?;
        Ok(())
    }
}
