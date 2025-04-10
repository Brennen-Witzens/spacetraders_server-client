use async_graphql::EmptySubscription;

use super::model::{MutationRoot, QueryRoot};

#[derive(Clone)]
pub struct Context {
    pub auth_token: String,
}

//impl Context {
//    pub fn auth_token(&self) -> &str {
//        &self.auth_token
//    }
//}

pub(crate) type Schema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub(crate) fn create_schema() -> Schema {
    async_graphql::Schema::new(QueryRoot, MutationRoot, EmptySubscription)
}
