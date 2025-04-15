use async_graphql::{Enum, InputObject, Object};
use serde::{Deserialize, Serialize};

use super::{contract::Contract, faction::Faction, ship::Ship};

// TODO:
// 1. Clean up values
// 2. Properly set values (some strings are date time strings and should be date times)
// 3. Get full graphql objects setup for these
// 4. Work on documentation of these in some fashion
// 5. Stretch Goal - database objects/architecture plans

#[derive(InputObject)]
pub struct NewAgent {
    symbol: String,
    faction: String,
    email: Option<String>,
}

impl NewAgent {
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn faction(&self) -> &str {
        &self.faction
    }

    pub fn email(&self) -> Option<&str> {
        self.email.as_deref()
    }
}

//TODO: Change this into a generic

///
/// struct DataResponse<T> {
///     data: <T>
/// }
/// struct Agent {
/// // code here
/// }
///
/// New Agent
/// DataResponse<Agent> {
///     data: <Agent>
/// }

#[derive(Serialize, Deserialize, Debug)]
pub struct NewAgentResponse {
    data: AgentResponse,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAgentResponse {
    data: Agent,
}

#[Object]
impl GetAgentResponse {
    // This is the 'root' of the response that Spacetraders returns as an OpenAPI spec
    // Since I'm mapping it via JSON, we need this root object
    async fn data(&self) -> &Agent {
        &self.data
    }
}
//#[derive(Serialize, Deserialize, Debug)]
//pub struct TestResponse<T> {
//    data: T,
//}

//#[Object]
//impl<T: async_graphql::OutputType> TestResponse<T> {
//    async fn data(&self) -> &T {
//        &self.data
//    }
//}

#[Object]
impl NewAgentResponse {
    // This is the 'root' of the response that Spacetraders returns as an OpenAPI spec
    // Since I'm mapping it via JSON, we need this root object
    async fn data(&self) -> &AgentResponse {
        &self.data
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentResponse {
    agent: Agent,
    contract: Contract,
    faction: Faction,
    ships: Vec<Ship>,
    token: String,
}

#[Object]
impl AgentResponse {
    async fn agent(&self) -> &Agent {
        &self.agent
    }
    async fn contract(&self) -> &Contract {
        &self.contract
    }
    async fn faction(&self) -> &Faction {
        &self.faction
    }
    async fn token(&self) -> &str {
        &self.token
    }
    async fn ships(&self) -> &Vec<Ship> {
        &self.ships
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    #[serde(skip_serializing_if = "Option::is_none")]
    account_id: Option<String>,
    symbol: String,
    headquarters: String,
    credits: i64,
    starting_faction: String,
    ship_count: i32,
}

#[Object]
impl Agent {
    async fn account_id(&self) -> Option<&str> {
        self.account_id.as_deref()
    }
    async fn symbol(&self) -> &str {
        &self.symbol
    }
    async fn headquarters(&self) -> &str {
        &self.headquarters
    }
    async fn credits(&self) -> i64 {
        self.credits
    }
    async fn starting_faction(&self) -> &str {
        &self.starting_faction
    }
    async fn ship_count(&self) -> i32 {
        self.ship_count
    }
}
