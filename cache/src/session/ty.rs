use anyhow::{anyhow, Result};
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

use ty::auth::Session;

pub struct SessionCache<'a> {
    pub(crate) cache: &'a Surreal<Client>,
}

impl SessionCache<'_> {
    pub async fn insert_session(&self, session: Session) -> Result<Session> {
        let mut r = self
            .cache
            .query("RETURN fn::insert_session($secret, $user, $expires_on)")
            .bind(("secret", session.secret))
            .bind(("user", session.user))
            .bind(("expires_on", session.expires_on))
            .await?;
        let session: Option<Session> = r.take(0)?;
        session.ok_or_else(|| anyhow!("insert_session: none"))
    }

    pub async fn get_session_by_secret(&self, secret: String) -> Result<Session> {
        let mut r = self
            .cache
            .query("RETURN fn::get_session_by_secret($secret)")
            .bind(("secret", secret))
            .await?;

        let session: Option<Session> = r.take(0)?;
        session.ok_or_else(|| anyhow!("get_session_by_secret: none"))
    }
}
