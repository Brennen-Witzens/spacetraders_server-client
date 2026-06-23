use async_graphql::Enum;
use serde::{Deserialize, Serialize};

use super::chart::Chart;

#[derive(Serialize, Deserialize, Debug)]
pub struct WaypointSymbol {
    symbol: String,
}

#[derive(Serialize, Deserialize, Debug, Enum, Copy, PartialEq, Eq, Clone)]
pub enum WaypointType {
    PLANET,
    GASGIANT,
    MOON,
    ORBITALSTATION,
    JUMPGATE,
    ASTEROIDFIELD,
    ASTEROID,
    ENGINEEREDASTEROID,
    ASTEROIDBASE,
    NEBULA,
    DEBRISFIELD,
    GRAVITYWELL,
    ARTIFICIALGRAVITYWELL,
    FUELSTATION,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaypointOrbital {
    symbol: String,
}

#[derive(Serialize, Deserialize, Debug, Enum, Copy, PartialEq, Eq, Clone)]
pub enum WaypointFaction {
    COSMIC,
    VOID,
    GALACTIC,
    QUANTUM,
    DOMINION,
    ASTRO,
    CORSAIRS,
    OBSIDIAN,
    AEGIS,
    UNITED,
    SOLITARY,
    COBALT,
    OMEGA,
    ECHO,
    LORDS,
    CULT,
    ANCIENTS,
    SHADOW,
    ETHEREAL,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Waypoint {
    symbol: String,
    waypoint_type: WaypointType,
    system_symbol: String,
    x: i32,
    y: i32,
    orbitals: Vec<WaypointOrbital>,
    orbits: Option<String>,
    faction: Option<WaypointFaction>,
    traits: Vec<WaypointTrait>,
    modifiers: Option<Vec<WaypointModifier>>,
    chart: Option<Chart>,
    is_under_construction: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaypointTrait {
    symbol: WaypointTraitSymbol,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaypointModifier {
    symbol: WaypointModifierSymbol,
    name: String,
    description: String,
}

#[derive(Serialize, Deserialize, Debug, Enum, Copy, PartialEq, Eq, Clone)]
pub enum WaypointModifierSymbol {
    STRIPPED,
    UNSTABLE,
    RADIATIONLEAK,
    CRITICALLIMIT,
    CIVILUNREST,
}

#[derive(Serialize, Deserialize, Debug, Enum, Copy, PartialEq, Eq, Clone)]
pub enum WaypointTraitSymbol {
    UNCHARTED,
    UNDERCONSTRUCTION,
    MARKETPLACE,
    SHIPYARD,
    OUTPOST,
    SCATTEREDSETTLEMENTS,
    SPRAWLINGCITIES,
    MEGASTRUCTURES,
    PIRATEBASE,
    OVERCROWDED,
    HIGHTECH,
    CORRUPT,
    BUREAUCRATIC,
    TRADINGHUB,
    INDUSTRIAL,
    BLACKMARKET,
    RESEARCHFACILITY,
    MILITARYBASE,
    SURVEILLANCEOUTPOST,
    EXPLORATIONOUTPOST,
    MINERALDEPOSITS,
    COMMONMETALDEPOSITS,
    PRECIOUSMETALDEPOSITS,
    RAREMETALDEPOSITS,
    METHANEPOOLS,
    ICECRYSTALS,
    EXPLOSIVEGASES,
    STRONGMAGNETOSPHERE,
    VIBRANTAURORAS,
    SALTFLATS,
    CANYONS,
    PERPETUALDAYLIGHT,
    PERPETUALOVERCAST,
    DRYSEABEDS,
    MAGMASEAS,
    SUPERVOLCANOES,
    ASHCLOUDS,
    VASTRUINS,
    MUTATEDFLORA,
    TERRAFORMED,
    EXTREMETEMPERATURES,
    EXTREMEPRESSURE,
    DIVERSELIFE,
    SCARCELIFE,
    FOSSILS,
    WEAKGRAVITY,
    STRONGGRAVITY,
    CRUSHINGGRAVITY,
    TOXICATMOSPHERE,
    CORROSIVEATMOSPHERE,
    BREATHABLEATMOSPHERE,
    THINATMOSPHERE,
    JOVIAN,
    ROCKY,
    VOLCANIC,
    FROZEN,
    SWAMP,
    BARREN,
    TEMPERATE,
    JUNGLE,
    OCEAN,
    RADIOACTIVE,
    MICROGRAVITYANOMALIES,
    DEBRISCLUSTER,
    DEEPCRATERS,
    SHALLOWCRATERS,
    UNSTABLECOMPOSITION,
    HOLLOWEDINTERIOR,
    STRIPPED,
}
