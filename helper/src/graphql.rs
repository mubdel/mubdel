use async_graphql::{Context, Result};

use cache::conn::Cache;
use crypto::CryptoBuilder;
use database::conn::DB;
use errors::DiagCtx;
use payment_gate::stripe::Stripe;
use ty::role::Role;

pub fn get_db<'a>(ctx: &'a Context<'_>) -> Result<&'a DB> {
    match ctx.data::<DB>() {
        Ok(db) => Ok(db),
        Err(err) => Err(format!("database client not found: {}", err.message).into()),
    }
}

pub fn get_cache<'a>(ctx: &'a Context<'_>) -> Result<&'a Cache> {
    match ctx.data::<Cache>() {
        Ok(cache) => Ok(cache),
        Err(err) => Err(format!("cache client not found: {}", err.message).into()),
    }
}

pub fn get_diag<'a>(ctx: &'a Context<'_>) -> Result<&'a DiagCtx> {
    match ctx.data::<DiagCtx>() {
        Ok(diag) => Ok(diag),
        Err(err) => Err(format!("diagnostic context not found: {}", err.message).into()),
    }
}

pub fn get_role<'a>(ctx: &'a Context<'_>) -> Result<&'a Role> {
    match ctx.data::<Role>() {
        Ok(r) => Ok(r),
        Err(err) => Err(format!("user role not found: {}", err.message).into()),
    }
}

pub fn get_stripe<'a>(ctx: &'a Context<'_>) -> Result<&'a Stripe> {
    match ctx.data::<Stripe>() {
        Ok(r) => Ok(r),
        Err(err) => Err(format!("stripe client not found: {}", err.message).into()),
    }
}

pub fn get_crypto_builder<'a>(ctx: &'a Context<'_>) -> Result<&'a CryptoBuilder> {
    match ctx.data::<CryptoBuilder>() {
        Ok(r) => Ok(r),
        Err(err) => Err(format!("crypto builder not found: {}", err.message).into()),
    }
}
