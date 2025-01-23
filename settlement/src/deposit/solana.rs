use std::{
    thread::{self, sleep},
    time::Duration,
};

use solana_client::rpc_client::RpcClient;
use tracing::{error, info};

use database::conn::DB;

use crate::deposit::Depositor;

pub struct Solana {
    pub db: DB,
    pub rpc_client: RpcClient,
}

impl Solana {
    /// Create new solana depositor worker
    pub fn new_depositor(db: DB, rpc_client: RpcClient) -> Self {
        Self { db, rpc_client }
    }
}

impl Depositor for Solana {
    async fn start(&self) {
        info!("solana depositor started");

        loop {
            let crypto_db = self.db.crypto_db();

            let r = crypto_db.get_block("SOL".to_string()).await;
            if r.is_err() {
                error!(
                    "solana depositor: unable to get block from database: {:?}",
                    r.err()
                );
                continue;
            }
            let block = r.unwrap();
            if block.is_none() {
                error!("solana depositor: latest block not found in database");
                continue;
            }
            let block = block.unwrap();

            let r = block.latest_block.parse();
            if r.is_err() {
                error!(
                    "solana depositor: unable to parse latest block: {:?}",
                    r.err()
                );
                continue;
            }
            let latest_block: u64 = r.unwrap();

            let r = self.rpc_client.get_blocks(latest_block + 1, None);
            if r.is_err() {
                error!("solana depositor: unable to get blocks: {:?}", r.err());
                continue;
            }

            let blocks = r.unwrap();
            if blocks.is_empty() {
                info!("solana depositor: no new blocks to parse");
                continue;
            }

            let new_latest_block = blocks.last().unwrap().to_string();

            for block_id in blocks.into_iter() {
                thread::spawn(move || {
                    send_block(block_id);
                });
            }

            let r = crypto_db
                .update_latest_block("SOL".to_string(), new_latest_block)
                .await;
            if r.is_err() {
                error!(
                    "solana depositor: unable to update latest block: {:?}",
                    r.err()
                );
                continue;
            }

            sleep(Duration::from_secs(60 * 2));
        }
    }
}

/// Send solana block to blocks parser API
fn send_block(block_id: u64) {
    info!("solana depositor: send block `{}`", block_id);
}
