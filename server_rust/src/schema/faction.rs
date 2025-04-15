use async_graphql::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FactionTraits {
    symbol: String,
    name: String,
    description: String,
}

#[Object]
impl FactionTraits {
    async fn symbol(&self) -> &str {
        &self.symbol
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn description(&self) -> &str {
        &self.description
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Faction {
    symbol: String,
    name: String,
    description: String,
    headquarters: Option<String>,
    traits: Vec<FactionTraits>,
    is_recruiting: bool,
}

#[Object]
impl Faction {
    async fn symbol(&self) -> &str {
        &self.symbol
    }
    async fn name(&self) -> &str {
        &self.name
    }
    async fn description(&self) -> &str {
        &self.description
    }
    async fn traits(&self) -> &Vec<FactionTraits> {
        &self.traits
    }
    async fn is_recruiting(&self) -> bool {
        self.is_recruiting
    }
}
