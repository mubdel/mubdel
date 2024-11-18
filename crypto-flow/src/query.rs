use async_graphql::{Context, Error, Object};

#[derive(Copy, Clone)]
pub struct RootQuery;

#[Object]
impl RootQuery {
    async fn service(&self, _ctx: &Context<'_>) -> Result<&str, Error> {
        Ok("crypto_flow_service")
    }
}
