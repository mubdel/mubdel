use std::future::Future;

use anyhow::Result;

use crate::ty::{CheckoutSession, Customer, PaymentMethod};

pub mod stripe;
pub mod ty;

pub trait Gate {
    fn create_customer(&self, customer: Customer) -> impl Future<Output = Result<String>> + Send;
    fn create_checkout_session(
        &self,
        customer_id: String,
        price_id: String,
        seats: u64,
    ) -> impl Future<Output = Result<CheckoutSession>> + Send;
    fn create_payment_method(
        &self,
        method: PaymentMethod,
        customer_id: String,
    ) -> impl Future<Output = Result<String>> + Send;
    fn subscribe(
        &self,
        customer_id: String,
        price_id: String,
        payment_method_id: String,
        seats: u64,
    ) -> impl Future<Output = Result<String>> + Send;
    fn unsubscribe(&self, subscription_id: String) -> impl Future<Output = Result<()>> + Send;
}
