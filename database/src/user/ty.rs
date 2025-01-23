use anyhow::{anyhow, Result};
use surrealdb::{engine::remote::ws::Client, sql::thing, Surreal};

use ty::user::User;

pub struct UserDatabase<'a> {
    pub(crate) db: &'a Surreal<Client>,
}

impl UserDatabase<'_> {
    pub async fn insert_user(&self, user: User) -> Result<User> {
        let mut r = self
            .db
            .query("RETURN fn::insert_user($name, $username, $email, $password)")
            .bind(("name", user.name))
            .bind(("username", user.username))
            .bind(("email", user.email))
            .bind(("password", user.password))
            .await?;
        let user: Option<User> = r.take(0)?;
        user.ok_or_else(|| anyhow!("insert user: none"))
    }

    pub async fn get_user_by_email(&self, email: String) -> Result<User> {
        let mut r = self
            .db
            .query("RETURN fn::get_user_by_email($email)")
            .bind(("email", email))
            .await?;
        let user: Option<User> = r.take(0)?;
        user.ok_or_else(|| anyhow!("get_user_by_email: none"))
    }

    pub async fn get_user_by_username(&self, username: String) -> Result<User> {
        let mut r = self
            .db
            .query("RETURN fn::get_user_by_username($username)")
            .bind(("username", username))
            .await?;

        let user: Option<User> = r.take(0)?;
        user.ok_or_else(|| anyhow!("get_user_by_username: none"))
    }

    pub async fn get_user(&self, id: String) -> Result<User> {
        let mut r = self
            .db
            .query("RETURN fn::get_user($id)")
            .bind(("id", id))
            .await?;

        let user: Option<User> = r.take(0)?;
        user.ok_or_else(|| anyhow!("get_user: none"))
    }

    pub async fn delete_user(&self, user_id: String) -> Result<User> {
        let mut r = self
            .db
            .query("RETURN fn::delete_user($user_id)")
            .bind(("user_id", thing(&user_id)?))
            .await?;

        let user: Option<User> = r.take(0)?;
        user.ok_or_else(|| anyhow!("delete_user: none"))
    }

    pub async fn update_stripe_customer_id(
        &self,
        user_id: String,
        customer_id: String,
    ) -> Result<User> {
        let mut r = self
            .db
            .query("RETURN fn::update_stripe_customer_id($user_id, $customer_id)")
            .bind(("user_id", thing(&user_id)?))
            .bind(("customer_id", customer_id))
            .await?;

        let user: Option<User> = r.take(0)?;
        user.ok_or_else(|| anyhow!("update_stripe_customer_id: none"))
    }
}
