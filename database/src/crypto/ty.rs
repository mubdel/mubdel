use anyhow::{anyhow, Result};
use surrealdb::{engine::remote::ws::Client, Surreal};

use ty::block::Block;
use ty::wallet::Wallet;

pub struct CryptoDatabase<'a> {
    pub(crate) db: &'a Surreal<Client>,
}

impl<'a> CryptoDatabase<'a> {
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
}
