use async_graphql::{FieldError, Object};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hello_world(&self) -> Result<String, FieldError> {
        Ok("Hello world".to_string())
    }
}
