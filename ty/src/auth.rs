use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use poem::{FromRequest, Request, RequestBody, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, SimpleObject, InputObject, Default)]
#[graphql(input_name = "login_input")]
pub struct Login {
    pub email_or_username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, SimpleObject, InputObject, Default)]
#[graphql(input_name = "session_output")]
pub struct Session {
    pub secret: String,
    pub user: String,
    #[graphql(skip)]
    pub expires_on: DateTime<Utc>,
}

#[derive(Clone)]
pub struct SessionSecret(pub Option<String>);

impl<'a> FromRequest<'a> for SessionSecret {
    async fn from_request(req: &'a Request, _body: &mut RequestBody) -> Result<Self> {
        let s = req.extensions().get::<SessionSecret>();
        Ok(s.unwrap_or(&SessionSecret(None)).clone())
    }
}
