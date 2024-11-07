use serde::{Deserialize, Deserializer};
use surrealdb::sql::Thing;

pub fn thing_to_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let t = Thing::deserialize(deserializer)?;
    Ok(t.to_raw())
}
