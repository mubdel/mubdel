use async_graphql::{InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::surreal::thing_to_string;

#[derive(Debug, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "block_input")]
pub struct Block {
    #[serde(deserialize_with = "thing_to_string")]
    #[graphql(default)]
    pub id: String,
    pub currency: String,
    pub latest_block: String,
    #[graphql(skip)]
    pub created_at: Datetime,
}

/// Block statistics
#[derive(Debug, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "block_statistics_input")]
pub struct BlockStatistics {
    /// The number of transactions in the block associated with our wallets
    pub number_of_transactions: u64,
    /// The total amount received in the block
    pub received_amount: u64,
}
