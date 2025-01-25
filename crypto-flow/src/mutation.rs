use async_graphql::{Context, Object, Result};

use crypto::CryptoCurrency;
use errors::{errors as errs, gql_err};
use helper::graphql::{get_crypto_builder, get_db, get_diag, get_role};
use ty::{block::BlockStatistics, currency::CryptoCurrency as Currency, wallet::Wallet};

#[derive(Copy, Clone)]
pub struct RootMutation;

#[Object]
impl RootMutation {
    async fn get_wallet(&self, ctx: &Context<'_>, currency: Currency) -> Result<Wallet> {
        let role = get_role(ctx)?;
        let diag = get_diag(ctx)?;
        let (ok, user_role) = role.is_user();
        if !ok {
            gql_err!(diag, errs::Unauthorized {});
        }
        let user_id = user_role.unwrap().id;

        let db = get_db(ctx)?;

        let crypto_db = db.crypto_db();
        let symbol = currency.symbol().to_string();
        let wallet = crypto_db
            .get_latest_wallet(user_id.clone(), symbol.clone())
            .await?;
        if let Some(wallet) = wallet {
            return Ok(wallet);
        }

        let cb = get_crypto_builder(ctx)?;
        let crypto_inst = cb.build(currency)?;

        let w = crypto_inst.create_wallet().await;
        // FIXME: Encrypt the private key
        let wallet = crypto_db
            .insert_wallet(user_id, symbol, w.public_key, w.private_key)
            .await?;

        Ok(wallet)
    }

    async fn regenerate_wallet(&self, ctx: &Context<'_>, currency: Currency) -> Result<Wallet> {
        let role = get_role(ctx)?;
        let diag = get_diag(ctx)?;
        let (ok, user_role) = role.is_user();
        if !ok {
            gql_err!(diag, errs::Unauthorized {});
        }
        let user_id = user_role.unwrap().id;

        let db = get_db(ctx)?;

        let cb = get_crypto_builder(ctx)?;
        let crypto_inst = cb.build(currency)?;

        let w = crypto_inst.create_wallet().await;
        // FIXME: Encrypt the private key
        let symbol = currency.symbol().to_string();
        let wallet = db
            .crypto_db()
            .insert_wallet(user_id, symbol, w.public_key, w.private_key)
            .await?;

        Ok(wallet)
    }

    /// Parse transactions in a block
    async fn parse_block(
        &self,
        ctx: &Context<'_>,
        currency: Currency,
        block_id: String,
    ) -> Result<BlockStatistics> {
        let role = get_role(ctx)?;
        let diag = get_diag(ctx)?;
        if !role.is_system() {
            gql_err!(diag, errs::Unauthorized {});
        }

        let cb = get_crypto_builder(ctx)?;
        let crypto_inst = cb.build(currency)?;

        let statistics = crypto_inst.parse_block(block_id).await?;

        Ok(statistics)
    }
}
