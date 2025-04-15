use async_graphql::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Payment {
    on_accepted: i32,
    on_fulfilled: i32,
}

#[Object]
impl Payment {
    async fn on_accepted(&self) -> i32 {
        self.on_accepted
    }
    async fn on_fulfilled(&self) -> i32 {
        self.on_fulfilled
    }
}
