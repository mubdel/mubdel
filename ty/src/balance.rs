use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::surreal::thing_to_string;

#[derive(Debug, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "balance_input")]
pub struct Balance {
    #[serde(deserialize_with = "thing_to_string")]
    #[graphql(default)]
    pub id: String,
    #[serde(deserialize_with = "thing_to_string")]
    #[graphql(skip)]
    pub user: String,
    pub currency: String,
    pub amount: u64,
    #[graphql(skip)]
    pub created_at: Datetime,
}
