//! Configuration

use std::fs::read_to_string;

use anyhow::{anyhow, Result};
use serde::Deserialize;
use toml::from_str;

#[derive(Clone, Deserialize)]
pub struct Config {
    service: Service,
    database: Database,
    cache: Cache,
    fluent: Fluent,
    loki: Loki,
    email: Option<EmailCfg>,
    payment_gate: Option<PaymentGate>,
    solana_node: Option<SolanaNode>,
}

#[derive(Clone, Deserialize)]
pub struct Service {
    /// The service port
    pub port: u32,
}

#[derive(Clone, Deserialize)]
pub struct Fluent {
    /// The path to fluent messages file
    pub fluent_path: String,
}

#[derive(Clone, Deserialize)]
pub struct Database {
    /// The database address in `ip:port` format
    pub addr: String,
    /// The username of database
    pub username: String,
    /// The password of database
    pub password: String,
    /// The namespace of database
    pub namespace: String,
    /// The name of database
    pub name: String,
}

#[derive(Clone, Deserialize)]
pub struct Cache {
    /// The cache address in `ip:port` format
    pub addr: String,
    /// The username of cache
    pub username: String,
    /// The password of cache
    pub password: String,
    /// The namespace of cache
    pub namespace: String,
    /// The name of cache
    pub name: String,
}

#[derive(Clone, Deserialize)]
pub struct EmailCfg {
    /// The path of email templates
    pub templates_path: String,
    /// AWS SES configuration
    pub aws: EmailAWS,
}

#[derive(Clone, Deserialize)]
pub struct EmailAWS {
    /// The AWS SES username
    pub username: String,
    /// The AWS SES password
    pub password: String,
    /// The AWS SES SMTP server
    pub smtp_server: String,
    /// The email address that should appear in the "From" field
    pub from: String,
}

#[derive(Clone, Deserialize)]
pub struct Loki {
    /// The Loki URL in `http://ip:port` format
    pub url: String,
}

#[derive(Clone, Deserialize)]
pub struct PaymentGate {
    /// The Stripe secret key
    pub stripe_secret: String,
    /// The Stripe webhook verification token
    pub stripe_wvt: String,
}

#[derive(Clone, Deserialize)]
pub struct SolanaNode {
    /// The Solana URL
    pub url: String,
}

impl Config {
    pub fn load_cfg(path: String) -> Result<Self> {
        let s = read_to_string(path)?;

        let c = from_str(&s)?;

        Ok(c)
    }

    pub fn service(&self) -> &Service {
        &self.service
    }

    pub fn fluent(&self) -> &Fluent {
        &self.fluent
    }

    pub fn db(&self) -> &Database {
        &self.database
    }

    pub fn cache_cfg(&self) -> &Cache {
        &self.cache
    }

    pub fn email_cfg(&self) -> Result<&EmailCfg> {
        match &self.email {
            Some(email) => Ok(email),
            None => Err(anyhow!("email configuration not found")),
        }
    }

    pub fn email_aws_cfg(&self) -> Result<&EmailAWS> {
        Ok(&self.email_cfg()?.aws)
    }

    pub fn loki_cfg(&self) -> &Loki {
        &self.loki
    }

    pub fn payment_gate(&self) -> Result<&PaymentGate> {
        match &self.payment_gate {
            Some(cfg) => Ok(cfg),
            None => Err(anyhow!("payment gate configuration not found")),
        }
    }

    pub fn solana_node(&self) -> Result<&SolanaNode> {
        match &self.solana_node {
            Some(cfg) => Ok(cfg),
            None => Err(anyhow!("solana node configuration not found")),
        }
    }
}
