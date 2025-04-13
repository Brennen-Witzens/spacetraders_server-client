use async_graphql::EmptySubscription;

use super::mutation_root::MutationRoot;
use super::query_root::QueryRoot;

#[derive(Clone)]
pub struct Context {
    pub account_token: String,
}

//impl Context {
//    pub fn auth_token(&self) -> &str {
//        &self.auth_token
//    }
//}

// TODO: make (web) request client module/util
// Builder of some sort, etc.
pub const API_URL: &str = "https://api.spacetraders.io/v2/";

pub(crate) type Schema = async_graphql::Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub(crate) fn create_schema() -> Schema {
    async_graphql::Schema::new(QueryRoot, MutationRoot, EmptySubscription)
}
