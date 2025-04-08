use std::collections::HashMap;

use actix_web::web::Json;
use async_graphql::{
    Context, EmptyMutation, EmptySubscription, Enum, ErrorExtensions, FieldError, ID, InputObject,
    Object, Schema,
};
use serde::{Deserialize, Serialize};
use serde_json::from_reader;
use thiserror::Error;

pub type BooksSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

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

#[derive(Clone, Debug)]
pub struct Book {
    id: ID,
    name: String,
    author: String,
}

#[Object]
impl Book {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn author(&self) -> &str {
        &self.author
    }
}

#[derive(InputObject)]
pub struct NewUser {
    symbol: String,
    faction: String,
    email: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterNewUserResponse {
    data: Data,
}

#[Object]
impl RegisterNewUserResponse {
    async fn data(&self) -> &Data {
        &self.data
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    agent: Agent,
    contract: Contract,
    faction: Faction,
    ships: Vec<Ships>,
    token: String,
}

#[Object]
impl Data {
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
    async fn ships(&self) -> &Vec<Ships> {
        &self.ships
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ships {
    symbol: String,
    registration: Registration,
    nav: ShipNav,
    crew: ShipCrew,
    frame: ShipFrame,
    reactor: ShipReactor,
    engine: ShipEngine,
    cooldown: Cooldown,
    modules: Vec<ShipModule>,
    mounts: Vec<ShipMount>,
    cargo: ShipCargo,
    fuel: ShipFuel,
}

#[Object]
impl Ships {
    async fn symbol(&self) -> &str {
        &self.symbol
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Registration {
    name: String,
    faction_symbol: String,
    role: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipNav {
    system_symbol: String,
    waypoint_symbol: String,
    route: ShipNavRoute,
    status: String,
    flight_mode: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipNavRoute {
    destination: ShipNavRouteWaypoint,
    origin: ShipNavRouteWaypoint,
    departure_time: String,
    arrival: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipNavRouteWaypoint {
    symbol: String,
    #[serde(rename = "type")]
    waypoint_type: String,
    system_symbol: String,
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipCrew {
    current: i32,
    required: i32,
    capacity: i32,
    rotation: String, //Enum
    morale: i32,
    wages: i32,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipFrame {
    symbol: String, //enum
    name: String,
    description: String,
    condition: f64,
    integrity: f64,
    module_slots: i32,
    mounting_points: i32,
    fuel_capacity: i32,
    requirements: ShipRequirements,
    quality: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipRequirements {
    #[serde(skip_serializing_if = "Option::is_none")]
    power: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    crew: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    slots: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipReactor {
    symbol: String,
    name: String,
    description: String,
    condition: f64,
    integrity: f64,
    power_output: i32,
    requirements: ShipRequirements,
    quality: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ShipEngine {
    symbol: String,
    name: String,
    description: String,
    condition: f64,
    integrity: f64,
    speed: i32,
    requirements: ShipRequirements,
    quality: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Cooldown {
    ship_symbol: String,
    total_seconds: i32,
    remaining_seconds: i32,
    #[serde(skip_deserializing)]
    expiration: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipModule {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity: Option<i32>,
    #[serde(skip_deserializing)]
    range: Option<i32>,
    name: String,
    description: String,
    requirements: ShipRequirements,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipMount {
    symbol: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strength: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    deposits: Option<Vec<String>>,
    requirements: ShipRequirements,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipCargo {
    capacity: i32,
    units: i32,
    inventory: Vec<ShipCargoItem>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipCargoItem {
    symbol: String,
    name: String,
    description: String,
    units: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipFuel {
    current: i32,
    capacity: i32,
    consumed: Option<ShipFuelData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipFuelData {
    amount: i32,
    timestamp: String, //date-time
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    #[serde(skip_deserializing)]
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

#[derive(Serialize, Enum, Copy, Clone, Eq, PartialEq, Deserialize, Debug)]
pub enum ContractType {
    PROCUREMENT,
    TRANSPORT,
    SHUTTLE,
}

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractDeliver {
    trade_symbol: String,
    destination_symbol: String,
    units_required: i32,
    units_fulfilled: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ContractTerms {
    deadline: String,
    payment: Payment,
    deliver: Option<Vec<ContractDeliver>>,
}

#[Object]
impl ContractTerms {
    async fn deadline(&self) -> &str {
        &self.deadline
    }
    async fn payment(&self) -> &Payment {
        &self.payment
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Contract {
    id: String,
    faction_symbol: String,
    #[serde(rename = "type")]
    contract_type: ContractType,
    terms: ContractTerms,
    accepted: bool,
    fulfilled: bool,
    #[serde(skip_deserializing)]
    deadline_to_accept: Option<String>,
}

#[Object]
impl Contract {
    async fn id(&self) -> &str {
        &self.id
    }
    async fn faction_symbol(&self) -> &str {
        &self.faction_symbol
    }
    async fn contract_type(&self) -> &ContractType {
        &self.contract_type
    }
    async fn accepted(&self) -> bool {
        self.accepted
    }
    async fn fulfilled(&self) -> bool {
        self.fulfilled
    }
    async fn deadline_to_accept(&self) -> Option<&str> {
        self.deadline_to_accept.as_deref()
    }
}

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

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn book(&self, _ctx: &Context<'_>) -> Book {
        let book: Book = Book {
            id: ID("0".to_string()),
            name: "A book".to_string(),
            author: "An unknown author".to_string(),
        };
        book
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
        //TODO: Make sure to not commit this :)
        //Need to pass this via context
        let bearer_auth = &context.auth_token;

        let mut map = HashMap::new();
        map.insert("symbol", user_data.symbol);
        map.insert("faction", user_data.faction);
        if let Some(email) = user_data.email {
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
