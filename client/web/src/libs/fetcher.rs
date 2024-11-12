use anyhow::{Error, Result};
use dioxus_logger::tracing::error;
use gql_client::Client;
use serde::{Deserialize, Serialize};

// FIXME: Use federation gate (gqlgate)
pub enum Service {
    User,
    Auth,
    Payment,
    CryptoFlow,
    Card,
}

const USER_BASE_URL: &str = "http://localhost:4001";
const AUTH_BASE_URL: &str = "http://localhost:4002";
const PAYMENT_BASE_URL: &str = "http://localhost:4003";
const CRYPTO_FLOW_BASE_URL: &str = "http://localhost:4004";
const CARD_BASE_URL: &str = "http://localhost:4006";

pub async fn fetch<V, D>(query: &str, vars: V, service: Service) -> Result<D>
where
    V: Serialize,
    for<'de> D: Deserialize<'de>,
{
    let url = match service {
        Service::User => USER_BASE_URL,
        Service::Auth => AUTH_BASE_URL,
        Service::Payment => PAYMENT_BASE_URL,
        Service::CryptoFlow => CRYPTO_FLOW_BASE_URL,
        Service::Card => CARD_BASE_URL,
    };

    let client = Client::new(url);
    client
        .query_with_vars_unwrap::<D, V>(query, vars)
        .await
        .map_err(|e| {
            error!("{}", e);
            Error::msg(format!("{}", e))
        })
}
