use std::collections::HashMap;

use async_graphql::{Context, Object, Result};

use errors::{errors as errs, gql_err};
use helper::graphql::{get_db, get_diag, get_role, get_stripe};
use payment_gate::{ty::Customer, Gate};
use ty::payment::{Subscribe, SubscriptionPeriod};

#[derive(Copy, Clone)]
pub struct RootMutation;

#[Object]
impl RootMutation {
    async fn subscribe(&self, ctx: &Context<'_>, subscribe: Subscribe) -> Result<String> {
        // FIXME: Make sure is user not guest
        let db = get_db(ctx)?;
        let plan = db.payment_db().get_plan_by_name(subscribe.plan).await?;

        let stripe = get_stripe(ctx)?;
        let price_id = if matches!(subscribe.period, SubscriptionPeriod::Yearly) {
            plan.yearly_stripe_price
        } else {
            plan.monthly_stripe_price
        };

        let role = get_role(ctx)?;
        let diag = get_diag(ctx)?;
        let (ok, user_role) = role.is_user();
        if !ok {
            gql_err!(diag, errs::Unauthorized {});
        }
        let user_role = user_role.unwrap();

        let user = db.user_db().get_user(user_role.id).await?;
        let customer_id = if let Some(customer_id) = user.stripe_customer_id {
            customer_id
        } else {
            let cus_id = stripe
                .create_customer(Customer {
                    email: user.email,
                    description: user.username,
                    metadata: HashMap::from([(String::from("user_id"), user.id.clone())]),
                    ..Default::default()
                })
                .await?;

            db.user_db()
                .update_stripe_customer_id(user.id.clone(), cus_id.clone())
                .await?;

            cus_id
        };

        let cs = stripe
            .create_checkout_session(customer_id, price_id, subscribe.seats)
            .await?;

        Ok(cs.checkout_url)
    }
}
