use async_graphql::{Context, ErrorExtensions, FieldError, Object};

use super::{agent::GetAgentResponse, context::API_URL, model::MyError};

pub struct QueryRoot;

// TODO:
// Each endpoint should call a function on a client or object that then does the actual request to
// the spacetraders endpoints. And all the resolver does is convert it into a GraphQL object
// it'll also help with not having to worry about creating multiple clients
#[Object]
impl QueryRoot {
    async fn hello_world(&self) -> Result<String, FieldError> {
        Ok("Hello world".to_string())
    }

    // TODO:
    // Account token is incorrect for the `get-agent` endpoint
    // Part of the request needs to handle the agent token
    async fn get_agent(&self, ctx: &Context<'_>) -> Result<GetAgentResponse, FieldError> {
        let context = ctx.data::<crate::Context>()?;

        let client = reqwest::Client::new();
        let endpoint_url = format!("{}{}", API_URL, "my/agent");
        let auth_header = format!("Bearer {}", &context.account_token);

        let res = client
            .get(endpoint_url)
            .header("Authorization", auth_header)
            .send()
            .await
            .map_err(|err| MyError::ServerError(err.to_string()).extend())?;

        let text = res
            .json::<GetAgentResponse>()
            .await
            .map_err(|err| MyError::ServerError(err.to_string()).extend())?;

        Ok(text)
    }
}
