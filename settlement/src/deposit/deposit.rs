use std::future::Future;

use anyhow::Result;

use ty::currency::CryptoCurrency as Currency;

use crate::deposit::solana::Solana;

pub fn new(currency: Currency) -> impl Depositor {
    match currency {
        Currency::SOL => Solana {},
        _ => Solana {}, // FIXME
    }
}

pub trait Depositor {
    /// Start deposit worker
    fn start(&self) -> impl Future<Output = Result<()>> + Send;
}
