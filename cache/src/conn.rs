use anyhow::Result;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;

use cfg::Cache as CacheCfg;

#[derive(Clone)]
pub struct Cache {
    pub cache: Surreal<Client>,
}

impl Cache {
    pub fn new(cache: Surreal<Client>) -> Self {
        Self { cache }
    }
}

pub async fn connect(cfg: &CacheCfg) -> Result<Cache> {
    let cache = Surreal::new::<Ws>(&cfg.addr).await?;

    cache
        .signin(Root {
            username: &cfg.username,
            password: &cfg.password,
        })
        .await?;

    cache.use_ns(&cfg.namespace).use_db(&cfg.name).await?;

    Ok(Cache::new(cache))
}
