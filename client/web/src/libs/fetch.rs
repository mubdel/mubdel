use anyhow::{Error, Result};
use gql_client::Client;
use serde::{Deserialize, Serialize};

pub async fn fetch<'a, T, D>(endpoint: &str, query: &str, vars: T) -> Result<D>
where
    T: Serialize,
    for<'de> D: Deserialize<'de>,
{
    let client = Client::new(endpoint);
    client
        .query_with_vars_unwrap::<D, T>(query, vars)
        .await
        .map_err(|e| Error::msg(format!("fetch gql error: {}", e)))
}
