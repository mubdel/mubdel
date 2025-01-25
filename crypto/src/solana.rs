use std::str::FromStr;
use std::sync::Arc;

use anyhow::Result;
use solana_client::{rpc_client::RpcClient, rpc_config::RpcBlockConfig};
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer, system_transaction};
use solana_transaction_status_client_types::{
    EncodedTransaction, TransactionDetails, UiTransactionEncoding,
};
use tracing::info;

use database::conn::DB;
use ty::block::BlockStatistics;

use crate::{CryptoCurrency, Wallet};

pub struct Solana {
    rpc_client: Arc<RpcClient>,
    db: Arc<DB>,
}

impl Solana {
    pub fn new(rpc_client: Arc<RpcClient>, db: Arc<DB>) -> Self {
        Self { rpc_client, db }
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
        let block_id: u64 = block_id.parse()?;

        let config = RpcBlockConfig {
            encoding: Some(UiTransactionEncoding::JsonParsed),
            transaction_details: Some(TransactionDetails::Accounts),
            rewards: Some(false),
            commitment: None,
            max_supported_transaction_version: Some(0),
        };
        let block = self.rpc_client.get_block_with_config(block_id, config)?;

        let (mut number_of_transactions, mut received_amount) = (0, 0);

        if let Some(trans) = block.transactions {
            for t in trans.iter() {
                if let EncodedTransaction::Accounts(accs) = &t.transaction {
                    for (index, ac) in accs.account_keys.iter().enumerate() {
                        let crypto_db = self.db.crypto_db();
                        if let Some(wallet) = crypto_db
                            .get_wallet_by_public_key(ac.pubkey.clone())
                            .await?
                        {
                            if let Some(meta) = &t.meta {
                                // FEATURE: let_chains
                                let amount = meta.post_balances[index] - meta.pre_balances[index];

                                crypto_db
                                    .insert_deposit(wallet.user.clone(), wallet.id, amount)
                                    .await?;

                                crypto_db
                                    .update_balance(
                                        wallet.user,
                                        wallet.currency,
                                        amount.try_into()?,
                                    )
                                    .await?;

                                number_of_transactions += 1;
                                received_amount += amount;
                            }
                        }
                    }
                }
            }
        }

        info!(
            "parse solana block: block ID `{}`, `{}` transactions found, `{}` received amount",
            block_id, number_of_transactions, received_amount
        );

        Ok(BlockStatistics {
            number_of_transactions,
            received_amount,
        })
    }
}
