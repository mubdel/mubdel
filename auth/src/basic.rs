use async_graphql::{Context, Object, Result};
use bcrypt::verify;
use chrono::{Duration, Utc};

use errors::errors as errs;
use errors::gql_err;
use helper::{
    graphql::{get_cache, get_db, get_diag},
    rand::generate_session,
};
use ty::auth::{Login, Session};

#[derive(Default, Copy, Clone)]
pub struct BasicAuthMutation;

#[Object]
impl BasicAuthMutation {
    async fn login(&self, ctx: &Context<'_>, login: Login) -> Result<Session> {
        let db = get_db(ctx)?;
        let cache = get_cache(ctx)?;
        let diag = get_diag(ctx)?;

        let user = if login.email_or_username.contains('@') {
            db.user_db()
                .get_user_by_email(login.email_or_username)
                .await?
        } else {
            db.user_db()
                .get_user_by_username(login.email_or_username)
                .await?
        };

        if !verify(login.password, &user.password)? {
            gql_err!(
                diag,
                errs::InvalidCredential {
                    field1: "EmailOrUsername",
                    field2: "Password",
                }
            );
        }

        let secret = generate_session();

        let session = Session {
            secret,
            user: user.id,
            expires_on: Utc::now() + Duration::hours(8),
        };

        let session = cache.session_cache().insert_session(session).await?;

        Ok(session)
    }
}
