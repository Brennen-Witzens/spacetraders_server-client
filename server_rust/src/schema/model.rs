use async_graphql::{ErrorExtensions, FieldError};
use thiserror::Error;

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
