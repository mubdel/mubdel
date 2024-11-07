use std::str::FromStr;

use anyhow::{bail, Result};
use rand::{distributions::Alphanumeric, rngs::OsRng, Rng};
use stripe::Client;

use crate::ty::{CheckoutSession, PaymentMethod};
use crate::{Customer, Gate};

#[derive(Clone)]
pub struct Stripe {
    client: Client,
    wvt: String,
}

impl Stripe {
    pub fn new(secret: String, wvt: String) -> Self {
        let client = Client::new(secret);

        Self { client, wvt }
    }
}

impl Gate for Stripe {
    async fn create_customer(&self, customer: Customer) -> Result<String> {
        let cus = stripe::Customer::create(
            &self.client,
            stripe::CreateCustomer {
                name: Some(&customer.name),
                email: Some(&customer.email),
                description: Some(&customer.description),
                address: Some(stripe::Address {
                    city: Some(customer.city),
                    country: Some(customer.country),
                    line1: Some(customer.addr_line1),
                    line2: Some(customer.addr_line2),
                    postal_code: Some(customer.postal_code),
                    state: Some(customer.state),
                }),
                metadata: Some(customer.metadata),
                ..Default::default()
            },
        )
        .await?;

        Ok(cus.id.to_string())
    }

    async fn create_checkout_session(
        &self,
        customer_id: String,
        price_id: String,
        seats: u64,
    ) -> Result<CheckoutSession> {
        let mut params = stripe::CreateCheckoutSession::new();
        params.customer = Some(stripe::CustomerId::from_str(&customer_id)?);
        params.mode = Some(stripe::CheckoutSessionMode::Subscription);
        params.expand = &[
            "line_items",
            "line_items.data.price.product",
            "subscription",
            "subscription.default_payment_method",
        ];
        params.line_items = Some(vec![stripe::CreateCheckoutSessionLineItems {
            quantity: Some(seats),
            price: Some(price_id),
            ..Default::default()
        }]);
        let session = generate_checkout_session();
        let success_url = format!(
            "https://api.mubdel.com/api/stripe_callback?wvt={}&session={}",
            self.wvt, session
        );
        params.success_url = Some(&success_url);
        params.cancel_url = Some("https://mubdel.com");

        let cs = stripe::CheckoutSession::create(&self.client, params).await?;

        let checkout_url = if let Some(checkout_url) = cs.url {
            checkout_url
        } else {
            bail!("checkout session: none checkout url");
        };

        Ok(CheckoutSession {
            id: cs.id.to_string(),
            checkout_url,
            session,
        })
    }

    async fn create_payment_method(
        &self,
        method: PaymentMethod,
        customer_id: String,
    ) -> Result<String> {
        let m = match method {
            PaymentMethod::Card(card) => stripe::CreatePaymentMethod {
                type_: Some(stripe::PaymentMethodTypeFilter::Card),
                card: Some(stripe::CreatePaymentMethodCardUnion::CardDetailsParams(
                    stripe::CardDetailsParams {
                        number: card.number,
                        exp_year: card.exp_year,
                        exp_month: card.exp_month,
                        cvc: Some(card.cvc),
                    },
                )),
                ..Default::default()
            },
        };

        let pm = stripe::PaymentMethod::create(&self.client, m).await?;

        stripe::PaymentMethod::attach(
            &self.client,
            &pm.id,
            stripe::AttachPaymentMethod {
                customer: stripe::CustomerId::from_str(&customer_id)?,
            },
        )
        .await?;

        Ok(pm.id.to_string())
    }

    async fn subscribe(
        &self,
        customer_id: String,
        price_id: String,
        payment_method_id: String,
        seats: u64,
    ) -> Result<String> {
        let mut params =
            stripe::CreateSubscription::new(stripe::CustomerId::from_str(&customer_id)?);
        let method_id = stripe::PaymentMethodId::from_str(&payment_method_id)?;
        params.default_payment_method = Some(&method_id);
        params.items = Some(vec![stripe::CreateSubscriptionItems {
            price: Some(price_id),
            quantity: Some(seats),
            ..Default::default()
        }]);
        let s = stripe::Subscription::create(&self.client, params).await?;

        Ok(s.id.to_string())
    }

    async fn unsubscribe(&self, subscription_id: String) -> Result<()> {
        stripe::Subscription::cancel(
            &self.client,
            &stripe::SubscriptionId::from_str(&subscription_id)?,
            stripe::CancelSubscription::default(),
        )
        .await?;

        Ok(())
    }
}

pub fn generate_checkout_session() -> String {
    OsRng
        .sample_iter(Alphanumeric)
        .take(35)
        .map(char::from)
        .collect()
}
