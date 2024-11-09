use anyhow::{Error, Result};
use gql_client::Client;
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "http://localhost:4001";

pub async fn fetch<V, D>(query: &str, vars: V) -> Result<D>
where
    V: Serialize,
    for<'de> D: Deserialize<'de>,
{
    let client = Client::new(BASE_URL);
    client
        .query_with_vars_unwrap::<D, V>(query, vars)
        .await
        .map_err(|e| Error::msg(format!("fetch gql error: {}", e)))
}
