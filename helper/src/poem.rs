use async_graphql::Result;
use poem::Request;

use cache::conn::Cache;
use database::conn::DB;

pub fn get_db(req: &Request) -> Result<&DB> {
    match req.data::<DB>() {
        Some(db) => Ok(db),
        None => Err("database client not found".into()),
    }
}

pub fn get_cache(req: &Request) -> Result<&Cache> {
    match req.data::<Cache>() {
        Some(c) => Ok(c),
        None => Err("cache client not found".into()),
    }
}
