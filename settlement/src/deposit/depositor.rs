use std::future::Future;

use solana_client::rpc_client::RpcClient;

use database::conn::DB;
use ty::currency::CryptoCurrency as Currency;

use crate::deposit::solana::Solana;

pub fn new(currency: Currency, db: DB, solana_rpc: RpcClient) -> impl Depositor {
    match currency {
        Currency::SOL => Solana { db, solana_rpc },
        _ => Solana { db, solana_rpc }, // FIXME
    }
}

pub trait Depositor {
    /// Start deposit worker
    fn start(&self) -> impl Future<Output = ()> + Send;
}
