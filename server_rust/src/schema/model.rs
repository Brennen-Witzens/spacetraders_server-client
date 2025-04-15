use async_graphql::{ErrorExtensions, FieldError};
use thiserror::Error;

// TODO:
// 1. Need to get models setup for what the client should send
// 2. Need to work on getting the OpenAPI spec side generated to be able to call
// 3. Database setup?

// TODO:
// 1. Work on a 'List' response generic
// Example being
//
//  ListResponse<T> {
//      data: Vec<T>,
//      meta: PaginationInfo,
//  }
//
//  struct PaginationInfo {
//      total: i32, // min 0, shows total amount of items of this kind that exist
//      page: i32, // min 1, default 1, a page that denotes an amount of items, offset from the
//      first item. each page holds an amount of items equal to the `limit`
//      limit: i32, // min 1, max 20, default 10, amount of items on each page. Limits how many
//      items can be fetched at once
//  }
//
//  // List Factions endpoint
//  ListResponse<Faction> {
//      data: Vec<Faction>,
//      meta: PaginationInfo
//  }
//
//  // List Contracts endpoint
//  ListResponse<Contract> {
//      data: Vec<Contract>,
//      meta: PaginationInfo
//  }
//

// TODO: Work on better error handling
// Current error handling is really bad
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
