use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};

#[derive(Deserialize, Serialize, Debug)]
pub struct AgentResponse {
    pub data: Agent,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Agent {
    accountId: String,
    symbol: String,
    headquarters: String,
    credits: i64,
    startingFaction: String,
    shipCount: u32,
}
impl Display for Agent {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = format!(
            "account id: {}\n\
            symbol: {}\n\
            headquarters: {}\n\
            credits: {}\n\
            starting faction: {}\n\
            ship count: {}\n\
            ",
            self.accountId,
            self.symbol,
            self.headquarters,
            self.credits,
            self.startingFaction,
            self.shipCount
        );
        write!(f, "{}", s)
    }
}

#[derive(Deserialize, Debug)]
pub struct ContractsResponse {
    pub data: Vec<Contract>,
    meta: Meta,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Contract {
    pub id: String,
    #[serde(rename = "factionSymbol")]
    pub faction_symbol: String,
    #[serde(rename = "type")]
    pub contract_type: String,
    pub terms: serde_json::Value, // Terms,
    pub accepted: bool,
    pub fulfilled: bool,
    pub expiration: String,
    #[serde(rename = "deadlineToAccept")]
    pub deadline_to_accept: String,
}
impl Display for Contract {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = format!(
            "id: {}\n\
            faction: {}\n\
            type: {}\n\
            accepted: {}\n\
            fulfilled: {}\n\
            expiration: {}\n\
            deadline to accept: {}\n\
            ",
            self.id,
            self.faction_symbol,
            self.contract_type,
            self.accepted,
            self.fulfilled,
            self.expiration,
            self.deadline_to_accept
        );
        write!(f, "{}", s)
    }
}

#[derive(Clone, Deserialize, Debug)]
struct Waypoint {
    symbol: String,
    #[serde(rename = "type")]
    waypoint_type: String,
    x: i32,
    y: i32,
    orbitals: Vec<serde_json::Value>,
}

#[derive(Deserialize, Debug)]
struct ShipsResponse {
    data: Vec<Ship>,
    meta: Meta,
}

#[derive(Clone, Deserialize, Debug)]
struct Ship {
    symbol: String,
    nav: serde_json::Value,
    crew: serde_json::Value,
    fuel: serde_json::Value,
    cooldown: serde_json::Value,
    frame: serde_json::Value,
    reactor: serde_json::Value,
    engine: serde_json::Value,
    modules: serde_json::Value,
    mounts: serde_json::Value,
    registration: serde_json::Value,
    cargo: serde_json::Value,
}
impl Display for Ship {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = format!("{}", self.symbol,);
        write!(f, "{}", s)
    }
}
impl Ship {
    fn details(&self) {
        let s = format!(
            "symbol: {}\n\
            crew: {}\n\
            fuel: {}\n\
            cooldown: {}\n\
            engine: {}\n\
            mounts: {}\n\
            cargo: {}\n\
            ",
            self.symbol,
            // self.nav,
            self.crew,
            self.fuel,
            self.cooldown,
            // self.frame,
            // self.reactor,
            self.engine,
            // self.modules,
            self.mounts,
            // self.registration,
            self.cargo,
        );
        println!("{}", s);
    }
}

#[derive(Deserialize, Debug)]
struct Meta {
    total: u32,
    page: u32,
    limit: u32,
}

#[derive(Deserialize, Debug)]
pub struct SystemsResponse {
    pub data: Vec<System>,
    meta: Meta,
}

#[derive(Clone, Deserialize, Debug)]
pub struct System {
    symbol: String,
    #[serde(rename = "sectorSymbol")]
    sector_symbol: String,
    #[serde(rename = "type")]
    system_type: String,
    x: i32,
    y: i32,
    waypoints: Vec<Waypoint>,
    factions: Vec<serde_json::Value>,
}
impl Display for System {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let s = format!("{}", self.symbol,);
        write!(f, "{}", s)
    }
}

#[derive(Deserialize, Debug)]
pub enum SingleContractResponse {
    SingleContractAccept,
    SingleContractError,
}

#[derive(Deserialize, Debug)]
pub struct SingleContractAccept {}

#[derive(Deserialize, Debug)]
pub struct SingleContractError {
    pub error: serde_json::Value,
}
