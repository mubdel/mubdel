use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use solana_client::rpc_client::RpcClient;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, system_transaction};

use ty::block::BlockStatistics;

use crate::{CryptoCurrency, Wallet};

pub struct Solana {
    rpc_client: Arc<RpcClient>,
}

impl Solana {
    pub fn new(rpc_client: Arc<RpcClient>) -> Self {
        Self { rpc_client }
    }
}

impl CryptoCurrency for Solana {
    async fn create_wallet(&self) -> Wallet {
        let keypair = Keypair::new();

        Wallet {
            public_key: keypair.pubkey().to_string(),
            private_key: keypair.to_base58_string(),
        }
    }

    async fn transfer(&self, sender: String, recipient: String, amount: u64) -> Result<()> {
        let to = Pubkey::from_str(&recipient)?;
        let from = Keypair::from_base58_string(&sender);
        let latest_blockhash = self.rpc_client.get_latest_blockhash()?;
        let tx = system_transaction::transfer(&from, &to, amount, latest_blockhash);
        self.rpc_client.send_and_confirm_transaction(&tx)?;
        Ok(())
    }

    async fn parse_block(&self, block_id: String) -> Result<BlockStatistics> {
        // TODO
        Ok(BlockStatistics {
            number_of_transactions: 0,
            received_amount: 0,
        })
    }
}
