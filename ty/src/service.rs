#[derive(Clone)]
pub enum Service {
    User,
    Auth,
    Payment,
    CryptoFlow,
}

impl Service {
    pub fn as_str(&self) -> &str {
        match self {
            Self::User => "user_service",
            Self::Auth => "auth_service",
            Self::Payment => "payment_service",
            Self::CryptoFlow => "crypto_flow_service",
        }
    }
}
