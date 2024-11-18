use async_graphql::Enum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Enum, Copy, Clone, Eq, PartialEq, Default)]
pub enum Currency {
    // Currencies
    /// United states dollar
    USD,
    /// Euro
    EUR,
    // Stable coins
    // Tether
    USDT,
    /// USD coin
    USDC,
    // Cryptocurrencies
    /// Bitcoin
    BTC,
    /// Ethereum
    ETH,
    #[default]
    // Solana
    SOL,
}

#[derive(Serialize, Deserialize, Debug, Enum, Copy, Clone, Eq, PartialEq, Default)]
pub enum CryptoCurrency {
    // Tether
    USDT,
    /// USD coin
    USDC,
    /// Bitcoin
    BTC,
    /// Ethereum
    ETH,
    #[default]
    // Solana
    SOL,
}

#[derive(Serialize, Deserialize, Debug, Enum, Copy, Clone, Eq, PartialEq, Default)]
pub enum CurrencyType {
    #[default]
    Currency,
    StableCoin,
    CryptoCurrency,
}

impl Currency {
    pub fn symbol(&self) -> &str {
        match self {
            Self::USD => "USD",
            Self::EUR => "EUR",
            Self::USDT => "USDT",
            Self::USDC => "USDC",
            Self::BTC => "BTC",
            Self::ETH => "ETH",
            Self::SOL => "SOL",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::USD => "united states dollar",
            Self::EUR => "euro",
            Self::USDT => "tether",
            Self::USDC => "USD coin",
            Self::BTC => "bitcoin",
            Self::ETH => "ethereum",
            Self::SOL => "solana",
        }
    }

    pub fn currency_type(&self) -> CurrencyType {
        match self {
            Self::USD | Self::EUR => CurrencyType::Currency,
            Self::USDT | Self::USDC => CurrencyType::StableCoin,
            Self::BTC | Self::ETH | Self::SOL => CurrencyType::CryptoCurrency,
        }
    }
}

impl CryptoCurrency {
    pub fn symbol(&self) -> &str {
        match self {
            Self::USDT => "USDT",
            Self::USDC => "USDC",
            Self::BTC => "BTC",
            Self::ETH => "ETH",
            Self::SOL => "SOL",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::USDT => "tether",
            Self::USDC => "USD coin",
            Self::BTC => "bitcoin",
            Self::ETH => "ethereum",
            Self::SOL => "solana",
        }
    }

    pub fn currency_type(&self) -> CurrencyType {
        match self {
            Self::USDT | Self::USDC => CurrencyType::StableCoin,
            Self::BTC | Self::ETH | Self::SOL => CurrencyType::CryptoCurrency,
        }
    }
}
