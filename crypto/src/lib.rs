use std::future::Future;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use solana_client::rpc_client::RpcClient;
use tracing::error;

use database::conn::DB;
use solana::Solana;
use ty::block::BlockStatistics;
use ty::currency::CryptoCurrency as Currency;

pub mod solana;

pub fn new() -> CryptoBuilder {
    CryptoBuilder {
        db: None,
        solana_rpc_client: None,
    }
}

pub trait CryptoCurrency {
    /// Create a new wallet
    fn create_wallet(&self) -> impl Future<Output = Wallet> + Send;
    /// Transfer from a wallet to another
    fn transfer(
        &self,
        sender: String,
        recipient: String,
        amount: u64,
    ) -> impl Future<Output = Result<()>> + Send;
    /// Parse transactions in a block
    fn parse_block(&self, block_id: String)
        -> impl Future<Output = Result<BlockStatistics>> + Send;
}

/// Used to build different crypto instances
pub struct CryptoBuilder {
    db: Option<Arc<DB>>,
    solana_rpc_client: Option<Arc<RpcClient>>,
}

impl CryptoBuilder {
    /// Set the database connection
    pub fn set_db(&mut self, db: DB) -> &mut Self {
        self.db = Some(Arc::new(db));
        self
    }

    /// Set solana rpc client
    pub fn set_solana_rpc(&mut self, rpc_client: RpcClient) -> &mut Self {
        self.solana_rpc_client = Some(Arc::new(rpc_client));
        self
    }

    /// Check if there are any unset values. If any value is unset, return an error;
    /// otherwise, return Ok
    pub fn check(&self) -> Result<()> {
        // TODO
        Ok(())
    }

    /// Get a crypto builder instance
    pub fn inst(&self) -> CryptoBuilder {
        CryptoBuilder {
            db: self.db.clone(),
            solana_rpc_client: self.solana_rpc_client.clone(),
        }
    }

    // Build a crypto currency instance
    pub fn build(&self, currency: Currency) -> Result<impl CryptoCurrency> {
        match currency {
            Currency::SOL => {
                if self.solana_rpc_client.is_none() {
                    error!("crypto builder: solana rpc client not set");
                    return Err(anyhow!("solana rpc client not set"));
                }
                if self.db.is_none() {
                    error!("crypto builder: solana database connection not set");
                    return Err(anyhow!("solana database connection not set"));
                }
                Ok(Solana::new(
                    self.solana_rpc_client.clone().unwrap(),
                    self.db.clone().unwrap(),
                ))
            }
            _ => Ok(Solana::new(
                self.solana_rpc_client.clone().unwrap(),
                self.db.clone().unwrap(),
            )), // FIXME
        }
    }
}

pub struct Wallet {
    pub public_key: String,
    pub private_key: String,
}
