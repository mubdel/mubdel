use async_graphql::{Enum, InputObject, SimpleObject};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Datetime;

use crate::surreal::thing_to_string;

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "subscribe_input")]
pub struct Subscribe {
    /// Plan name
    pub plan: String,
    /// Organization ID
    pub organization: String,
    /// Subscription period
    pub period: SubscriptionPeriod,
    /// Number of organization members
    pub seats: u64,
    // New card information
    // pub card: Option<NewCard>,
    // Saved card ID
    // pub saved_card: Option<String>,
}

#[derive(Serialize, Deserialize, Enum, Copy, Clone, Eq, PartialEq, Default)]
pub enum SubscriptionPeriod {
    #[default]
    Monthly,
    Yearly,
}

#[derive(Serialize, Deserialize, Enum, Copy, Clone, Eq, PartialEq, Default)]
pub enum PaymentMethodKind {
    #[default]
    Card,
    SavedCard,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "new_card_input")]
pub struct NewCard {
    pub name: String,
    pub number: String,
    pub exp_year: i32,
    pub exp_month: i32,
    pub cvc: String,
    /// Save card for later use
    pub save: bool,
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "plan_input")]
pub struct Plan {
    #[graphql(default)]
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
    pub name: String,
    pub monthly_price: i64,
    pub monthly_stripe_price: String,
    pub yearly_price: i64,
    pub yearly_stripe_price: String,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject)]
#[graphql(input_name = "card_input")]
pub struct Card {
    #[graphql(default)]
    #[serde(deserialize_with = "thing_to_string")]
    pub id: String,
    #[serde(deserialize_with = "thing_to_string")]
    #[graphql(default)]
    pub user: String,
    #[graphql(default)]
    pub last_four: String,
    pub stripe_id: String,
    pub label: Option<String>,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject, Default)]
pub struct Subscription {
    pub id: String,
    pub user: String,
    pub stripe_id: String,
    pub period: SubscriptionPeriod,
    pub price: i64,
    #[graphql(skip)]
    pub created_at: Datetime,
    pub canceled: bool,
    #[graphql(skip)]
    pub canceled_at: Option<Datetime>,
    pub plan: String,
    pub organization: String,
    pub seats: u64,
    pub payment_method: String,
    /// Used card
    pub card: Option<String>,
}
