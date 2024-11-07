use async_graphql::{EmptySubscription, Request, Response, Schema};
use poem::{
    handler,
    web::{Data, Json},
};

use ty::{auth::SessionSecret, role::Role};

use crate::mutation::RootMutation;
use crate::query::RootQuery;

pub type AuthSchema = Schema<RootQuery, RootMutation, EmptySubscription>;

#[handler]
pub async fn graphql_handler(
    schema: Data<&AuthSchema>,
    role: Role,
    session_secret: SessionSecret,
    req: Json<Request>,
) -> Json<Response> {
    Json(schema.execute(req.0.data(role).data(session_secret)).await)
}
