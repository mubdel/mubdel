use std::future::Future;

use anyhow::Result;

use solana::Solana;
use ty::currency::CryptoCurrency as Currency;

pub mod solana;

pub fn new(currency: Currency) -> impl CryptoCurrency {
    match currency {
        Currency::SOL => Solana {},
        _ => Solana {}, // FIXME
    }
}

pub trait CryptoCurrency {
    fn create_wallet(&self) -> impl Future<Output = Wallet> + Send;
    fn transfer(
        &self,
        sender: String,
        recipient: String,
        amount: u64,
    ) -> impl Future<Output = Result<()>> + Send;
}

pub struct Wallet {
    pub public_key: String,
    pub private_key: String,
}
