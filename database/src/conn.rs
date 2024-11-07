use anyhow::Result;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use cfg::Database;

#[derive(Clone)]
pub struct DB {
    pub db: Surreal<Client>,
}

impl DB {
    pub fn new(db: Surreal<Client>) -> Self {
        Self { db }
    }
}

pub async fn connect(cfg: &Database) -> Result<DB> {
    let db = Surreal::new::<Ws>(&cfg.addr).await?;

    db.signin(Root {
        username: &cfg.username,
        password: &cfg.password,
    })
    .await?;

    db.use_ns(&cfg.namespace).use_db(&cfg.name).await?;

    Ok(DB::new(db))
}
