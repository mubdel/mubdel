use std::collections::BTreeMap;

use anyhow::{Error, Result};
use async_graphql::{ServerError, Value};
use gql_client::Client;
use serde::{Deserialize, Deserializer, Serialize};

const BASE_URL: &str = "http://localhost:4001";

pub async fn fetch<T, D>(query: &str, vars: T) -> Result<D>
where
    T: Serialize,
    for<'de> D: Deserialize<'de>,
{
    let client = Client::new(BASE_URL);
    client
        .query_with_vars_unwrap::<D, T>(query, vars)
        .await
        .map_err(|e| Error::msg(format!("fetch gql error: {}", e)))
}

pub struct Data<T> {
    pub data: T,
    pub errors: Vec<ServerError>,
    pub extensions: BTreeMap<String, Value>,
}

impl<'de, T> Deserialize<'de> for Data<T>
where
    T: serde::de::DeserializeOwned,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let (data, errors, extensions) = serde::de::Deserialize::deserialize(deserializer)?;

        Ok(Data {
            data,
            errors,
            extensions,
        })
    }
}
