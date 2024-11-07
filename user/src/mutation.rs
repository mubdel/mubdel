use async_graphql::{Context, Object, Result};
use bcrypt::hash;

use errors::{errors as errs, gql_err};
use helper::graphql::{get_db, get_diag, get_role};
use ty::user::{User, HASH_COST};

#[derive(Copy, Clone)]
pub struct RootMutation;

#[Object]
impl RootMutation {
    async fn register(&self, ctx: &Context<'_>, mut user: User) -> Result<User> {
        let db = get_db(ctx)?;

        user.password = hash(user.password, HASH_COST)?;

        let created = db.user_db().insert_user(user).await?;

        // TODO: Send verification link to user email

        Ok(created)
    }

    async fn delete_my_account(&self, ctx: &Context<'_>) -> Result<User> {
        let diag = get_diag(ctx)?;
        let role = get_role(ctx)?;

        let (ok, user_role) = role.is_user();
        if !ok {
            gql_err!(diag, errs::Unauthorized {});
        }
        let user_role = user_role.unwrap();

        let db = get_db(ctx)?;

        let user = db.user_db().delete_user(user_role.id.clone()).await?;

        Ok(user)
    }
}
