use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::surreal::thing_to_string;

pub const HASH_COST: u32 = 14;

#[derive(Debug, Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "user_input")]
pub struct User {
    #[serde(deserialize_with = "thing_to_string")]
    #[graphql(default)]
    pub id: String,
    pub name: Option<String>,
    pub username: String,
    pub email: String,
    #[graphql(skip_output)]
    pub password: String,
    #[graphql(skip)]
    pub status: UserStatus,
    #[graphql(skip)]
    pub email_verified: bool,
    #[graphql(skip)]
    pub created_at: Datetime,
    #[graphql(skip)]
    pub updated_at: Option<Datetime>,
    #[graphql(skip)]
    pub deleted_at: Option<Datetime>,
    #[graphql(skip)]
    pub stripe_customer_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Enum, Copy, Clone, Eq, PartialEq, Default)]
pub enum UserStatus {
    #[default]
    Active,
    Inactive,
    Blocked,
    Suspended,
    Pending,
    Deleted,
}
