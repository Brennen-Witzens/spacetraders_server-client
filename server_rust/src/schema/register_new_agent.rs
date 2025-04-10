use async_graphql::{Enum, InputObject, Object};
use serde::{Deserialize, Serialize};

#[derive(InputObject)]
pub struct NewUser {
    symbol: String,
    faction: String,
    email: Option<String>,
}

impl NewUser {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RegisterNewUserResponse {
    data: DataResponse,
}

#[Object]
impl RegisterNewUserResponse {
    // This is the 'root' of the response that Spacetraders returns as an OpenAPI spec
    // Since I'm mapping it via JSON, we need this root object
    async fn data(&self) -> &DataResponse {
        &self.data
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataResponse {
    agent: Agent,
    contract: Contract,
    faction: Faction,
    ships: Vec<Ships>,
    token: String,
}

#[Object]
impl DataResponse {
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
