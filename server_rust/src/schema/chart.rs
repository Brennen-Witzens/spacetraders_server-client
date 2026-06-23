use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    waypoint_symbol: String,
    submitted_by: String,
    submitted_on: String, // Date-Time
}
