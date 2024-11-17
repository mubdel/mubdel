use std::future::Future;

use anyhow::Result;

pub mod solana;

pub struct Wallet {
    pub public_key: String,
    pub private_key: String,
}

pub trait CryptoCurrency {
    fn create_wallet() -> impl Future<Output = Wallet> + Send;
    fn transfer(
        &self,
        sender: String,
        recipient: String,
        amount: u64,
    ) -> impl Future<Output = Result<()>> + Send;
}
