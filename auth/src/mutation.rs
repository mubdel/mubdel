use async_graphql::MergedObject;

use crate::basic::BasicAuthMutation;

#[derive(Copy, Clone, MergedObject, Default)]
pub struct RootMutation(BasicAuthMutation);
