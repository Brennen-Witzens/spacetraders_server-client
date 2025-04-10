use async_graphql::{Context, ErrorExtensions, FieldError, Object};
use std::collections::HashMap;
use thiserror::Error;

use super::register_new_agent::{NewUser, RegisterNewUserResponse};

// TODO:
// 1. Need to get models setup for what the client should send
// 2. Need to work on getting the OpenAPI spec side generated to be able to call
// 3. Database setup?

#[derive(Debug, Error)]
pub enum MyError {
    #[error("Could not find resource")]
    NotFound,

    #[error("ServerError")]
    ServerError(String),

    #[error("No Extensions")]
    ErrorWithoutExtensions,
}

// TODO: Work on changing this up
// 'Extend' MyError somehow
impl ErrorExtensions for MyError {
    // lets define our base extensions
    fn extend(&self) -> FieldError {
        self.extend_with(|err, e| match err {
            MyError::NotFound => e.set("code", "NOT_FOUND"),
            MyError::ServerError(reason) => e.set("reason", reason.to_string()),
            MyError::ErrorWithoutExtensions => {}
        })
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello_world(&self) -> Result<String, FieldError> {
        Ok("Hello world".to_string())
    }
}

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
        let url = "https://api.spacetraders.io/v2/register";
        let bearer_auth = &context.auth_token;

        let mut map = HashMap::new();
        map.insert("symbol", user_data.symbol());
        map.insert("faction", user_data.faction());
        if let Some(email) = user_data.email() {
            map.insert("email", email);
            let res = client
                .post(url)
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
            .post(url)
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
