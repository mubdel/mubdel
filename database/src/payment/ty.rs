use anyhow::{anyhow, Result};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use ty::payment::{Card, Plan, Subscription};

pub struct PaymentDatabase<'a> {
    pub(crate) db: &'a Surreal<Client>,
}

impl<'a> PaymentDatabase<'a> {
    pub async fn get_plan_by_name(&self, name: String) -> Result<Plan> {
        let mut r = self
            .db
            .query("RETURN fn::get_plan_by_name($name)")
            .bind(("name", name))
            .await?;
        let plan: Option<Plan> = r.take(0)?;
        plan.ok_or_else(|| anyhow!("get_plan_by_name: none"))
    }

    /// Insert credit or debit card
    pub async fn insert_card(&self, card: Card) -> Result<Card> {
        let mut r = self
            .db
            .query("RETURN fn::insert_card($user, $last_four, $stripe_id, $label)")
            .bind(("user", card.user))
            .bind(("last_four", card.last_four))
            .bind(("stripe_id", card.stripe_id))
            .bind(("label", card.label))
            .await?;
        let card: Option<Card> = r.take(0)?;
        card.ok_or_else(|| anyhow!("insert_card: none"))
    }

    /// Get card by ID
    pub async fn get_card(&self, id: String) -> Result<Card> {
        let mut r = self
            .db
            .query("RETURN fn::get_card($id)")
            .bind(("id", id))
            .await?;

        let card: Option<Card> = r.take(0)?;
        card.ok_or_else(|| anyhow!("get_card: none"))
    }

    /// Insert a subscription
    pub async fn insert_subscription(&self, subscription: Subscription) -> Result<Subscription> {
        let mut r = self
            .db
            .query(
                r#"RETURN fn::insert_subscription($user, $stripe_id, $period, $price,
                $plan, $organization, $seats, $payment_method, $card"#,
            )
            .bind(("user", subscription.user))
            .bind(("stripe_id", subscription.stripe_id))
            .bind(("period", subscription.period))
            .bind(("price", subscription.price))
            .bind(("plan", subscription.plan))
            .bind(("organization", subscription.organization))
            .bind(("seats", subscription.seats))
            .bind(("payment_method", subscription.payment_method))
            .bind(("card", subscription.card))
            .await?;
        let s: Option<Subscription> = r.take(0)?;
        s.ok_or_else(|| anyhow!("insert_subscription: none"))
    }
}
