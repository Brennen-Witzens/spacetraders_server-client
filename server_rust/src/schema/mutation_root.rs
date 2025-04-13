use std::collections::HashMap;

use super::{
    context::API_URL,
    model::MyError,
    register_new_agent::{NewUser, RegisterNewUserResponse},
};
use async_graphql::{Context, ErrorExtensions, FieldError, Object};

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn register_new_user(
        &self,
        ctx: &Context<'_>,
        user_data: NewUser,
    ) -> Result<RegisterNewUserResponse, FieldError> {
        let context = ctx.data::<crate::Context>()?;

        let client = reqwest::Client::new();
        let endpoint_url = format!("{}{}", API_URL, "register");
        let bearer_auth = &context.account_token;

        let mut map = HashMap::new();
        map.insert("symbol", user_data.symbol());
        map.insert("faction", user_data.faction());
        if let Some(email) = user_data.email() {
            map.insert("email", email);
            let res = client
                .post(endpoint_url)
                .header("Content-Type", "application/json")
                .header("Authorization", bearer_auth)
                .json(&map)
                .send()
                .await
                .map_err(|err| MyError::ServerError(err.to_string()).extend())?;

            let text = res
                .json::<RegisterNewUserResponse>()
                .await
                .map_err(|err| MyError::ServerError(err.to_string()).extend())?;
            return Ok(text);
        }

        let res = client
            .post(endpoint_url)
            .header("Content-Type", "application/json")
            .header("Authorization", bearer_auth)
            .json(&map)
            .send()
            .await
            .map_err(|err| MyError::ServerError(err.to_string()).extend())?;

        let text = res
            .json::<RegisterNewUserResponse>()
            .await
            .map_err(|err| MyError::ServerError(err.to_string()).extend())?;
        Ok(text)
    }
}
