use async_graphql::Object;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Ship {
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
impl Ship {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    expiration: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShipModule {
    symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    capacity: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
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
