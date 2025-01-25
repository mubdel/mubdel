use anyhow::{anyhow, Result};
use surrealdb::{engine::remote::ws::Client, Surreal};

use ty::balance::Balance;
use ty::block::Block;
use ty::deposit::Deposit;
use ty::wallet::Wallet;

pub struct CryptoDatabase<'a> {
    pub(crate) db: &'a Surreal<Client>,
}

impl CryptoDatabase<'_> {
    pub async fn insert_wallet(
        &self,
        user: String,
        currency: String,
        public_addr: String,
        private_addr: String,
    ) -> Result<Wallet> {
        let mut r = self
            .db
            .query("RETURN fn::insert_wallet($user, $currency, $public_address, $private_address)")
            .bind(("user", user))
            .bind(("currency", currency))
            .bind(("public_address", public_addr))
            .bind(("private_address", private_addr))
            .await?;
        let wallet: Option<Wallet> = r.take(0)?;
        wallet.ok_or_else(|| anyhow!("insert wallet: none"))
    }

    pub async fn get_latest_wallet(
        &self,
        user: String,
        currency: String,
    ) -> Result<Option<Wallet>> {
        let mut r = self
            .db
            .query("RETURN fn::get_latest_wallet($user, $currency)")
            .bind(("user", user))
            .bind(("currency", currency))
            .await?;

        Ok(r.take(0)?)
    }

    pub async fn get_wallet_by_public_key(&self, public_key: String) -> Result<Option<Wallet>> {
        let mut r = self
            .db
            .query("RETURN fn::get_wallet_by_public_key($public_key)")
            .bind(("public_key", public_key))
            .await?;

        Ok(r.take(0)?)
    }

    pub async fn get_block(&self, currency: String) -> Result<Option<Block>> {
        let mut r = self
            .db
            .query("RETURN fn::get_block($currency)")
            .bind(("currency", currency))
            .await?;

        Ok(r.take(0)?)
    }

    /// Update latest handled block for a currency
    pub async fn update_latest_block(
        &self,
        currency: String,
        latest_block: String,
    ) -> Result<Option<Block>> {
        let mut r = self
            .db
            .query("RETURN fn::update_latest_block($currency, $latest_block)")
            .bind(("currency", currency))
            .bind(("latest_block", latest_block))
            .await?;

        Ok(r.take(0)?)
    }

    pub async fn insert_deposit(
        &self,
        user: String,
        wallet: String,
        amount: u64,
    ) -> Result<Deposit> {
        let mut r = self
            .db
            .query("RETURN fn::insert_deposit($user, $currency, $amount)")
            .bind(("user", user))
            .bind(("wallet", wallet))
            .bind(("amount", amount))
            .await?;
        let deposit: Option<Deposit> = r.take(0)?;
        deposit.ok_or_else(|| anyhow!("insert deposit: none"))
    }

    pub async fn update_balance(
        &self,
        user: String,
        currency: String,
        amount: i32,
    ) -> Result<Balance> {
        let mut r = self
            .db
            .query("RETURN fn::update_balance($user, $currency, $amount)")
            .bind(("user", user))
            .bind(("currency", currency))
            .bind(("amount", amount))
            .await?;

        let balance: Option<Balance> = r.take(0)?;
        balance.ok_or_else(|| anyhow!("insert balance: none"))
    }
}
