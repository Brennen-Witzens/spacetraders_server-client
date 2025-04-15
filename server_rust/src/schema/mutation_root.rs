use std::collections::HashMap;

use super::{
    agent::{AgentResponse, NewAgent, NewAgentResponse},
    context::API_URL,
    model::MyError,
};
use async_graphql::{Context, ErrorExtensions, FieldError, Object};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn new_test(&self) -> Result<String, FieldError> {
        Ok("New text".to_string())
    }
    async fn register_new_user(
        &self,
        ctx: &Context<'_>,
        user_data: NewAgent,
    ) -> Result<NewAgentResponse, FieldError> {
        let context = ctx.data::<crate::Context>()?;

        let client = reqwest::Client::new();
        let endpoint_url = format!("{}{}", API_URL, "register");
        let auth_header = format!("Bearer {}", &context.account_token);

        let mut map = HashMap::new();
        map.insert("symbol", user_data.symbol());
        map.insert("faction", user_data.faction());
        if let Some(email) = user_data.email() {
            map.insert("email", email);
            let res = client
                .post(endpoint_url)
                .header("Content-Type", "application/json")
                .header("Authorization", auth_header)
                .json(&map)
                .send()
                .await
                .map_err(|err| MyError::ServerError(err.to_string()).extend())?;

            let text = res
                .json::<NewAgentResponse>()
                .await
                .map_err(|err| MyError::ServerError(err.to_string()).extend())?;
            return Ok(text);
        } else {
            let res = client
                .post(endpoint_url)
                .header("Content-Type", "application/json")
                .header("Authorization", auth_header)
                .json(&map)
                .send()
                .await
                .map_err(|err| MyError::ServerError(err.to_string()).extend())?;

            let text = res
                .json::<NewAgentResponse>()
                .await
                .map_err(|err| MyError::ServerError(err.to_string()).extend())?;

            Ok(text)
        }
    }
}
