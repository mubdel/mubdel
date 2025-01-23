use std::future::Future;

pub trait Depositor {
    /// Start deposit worker
    fn start(&self) -> impl Future<Output = ()> + Send;
}
