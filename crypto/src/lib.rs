use std::future::Future;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use solana_client::rpc_client::RpcClient;
use tracing::error;

use solana::Solana;
use ty::currency::CryptoCurrency as Currency;

pub mod solana;

pub fn new() -> CryptoBuilder {
    CryptoBuilder {
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
    fn parse_block(&self, block_id: String) -> impl Future<Output = Result<()>> + Send;
}

/// Used to build different crypto instance
pub struct CryptoBuilder {
    solana_rpc_client: Option<Arc<RpcClient>>,
}

impl CryptoBuilder {
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

    pub fn inst(&self) -> CryptoBuilder {
        CryptoBuilder {
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
                Ok(Solana::new(self.solana_rpc_client.clone().unwrap()))
            }
            _ => Ok(Solana::new(self.solana_rpc_client.clone().unwrap())), // FIXME
        }
    }
}

pub struct Wallet {
    pub public_key: String,
    pub private_key: String,
}
