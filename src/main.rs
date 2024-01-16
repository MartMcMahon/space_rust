use inquire::{MultiSelect, Select, Text};
use serde::{Deserialize, Serialize};
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() {
    // let name = Text::new("What is your name?").prompt();

    // match name {
    //     Ok(name) => println!("Hello {}", name),
    //     Err(_) => println!("An error happened when asking for your name, try again later."),
    // }

    let saved_agent = read_auth();
    println!("creating client");
    let api = Api::new();

    let agent_res = api.view_agent(&saved_agent).await.unwrap();

    println!("{:?}", agent_res);

    println!("fetching contracts");
    let contracts_res: ContractsResponse = api.fetch_contracts(&saved_agent).await.unwrap();

    for contract in contracts_res.data {
        let contract: Contract = contract.into();
        println!("{}", contract);
    }

    let main_menu = Select::new(
        ">",
        [
            "ships",
            "loans",
            "systems",
            "structures",
            "upgrades",
            "goods",
        ]
        .to_vec(),
    )
    .prompt();
    match main_menu {
        Ok(main_menu) => match main_menu {
            "ships" => ships_menu(api, &saved_agent).await,
            // "loans" => loans_menu(),
            "systems" => systems_menu(api, &saved_agent).await,
            // "structures" => structures_menu(),
            // "upgrades" => upgrades_menu(),
            // "goods" => goods_menu(),
            _ => panic!("invalid option"),
        },
        Err(_) => println!("An error happened when asking for your name, try again later."),
    };
}

async fn systems_menu(api: Api, agent: &Agent) {
    println!("fetching systems...");
    let systems = api.fetch_systems(&agent).await.unwrap();

    let s = Select::new(">systems", systems.data.clone()).prompt();

    println!("{:?}", &s);
}

async fn ships_menu(api: Api, agent: &Agent) {
    let ships = api.fetch_ships(&agent).await.unwrap();

    let s = Select::new(">ships", ships.data.clone()).prompt();
    println!("{:?}", &s.unwrap().details());
}

// search
// let waypoint_shipyard_res = api
//     .search_waypoint(agent, "X1-RP15".to_string(), "SHIPYARD".to_string())
//     .await;

// println!("{:?}", waypoint_shipyard_res);
// }

fn read_auth() -> Agent {
    read_auth_file("auth.json".to_string()).unwrap()
}

fn read_auth_file(filename: String) -> Result<Agent, std::io::Error> {
    println!("reading {filename}");
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(serde_json::from_str(&contents)?)
}

#[derive(Deserialize, Debug)]
struct Agent {
    callsign: String,
    token: String,
}

struct Api {
    client: reqwest::Client,
    url: String,
}
impl Api {
    fn new() -> Api {
        Api {
            client: reqwest::Client::new(),
            url: "https://api.spacetraders.io/v2".to_string(),
        }
    }

    async fn view_agent(&self, agent: &Agent) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!("{}/my/agent", self.url);
        let res = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", agent.token))
            .send()
            .await?;
        let agent_res: serde_json::Value = res.json().await?;
        Ok(agent_res)
    }

    async fn fetch_contracts(&self, agent: &Agent) -> Result<ContractsResponse, reqwest::Error> {
        let url = format!("{}/my/contracts", self.url);
        let res = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", agent.token))
            .send()
            .await?;
        let contracts_res: ContractsResponse = res.json().await?;
        Ok(contracts_res)
    }

    async fn search_waypoint(
        &self,
        agent: &Agent,
        system: String,
        traits: String,
    ) -> Result<serde_json::Value, reqwest::Error> {
        let url = format!(
            "{}/systems/{}/waypoints?traits={}",
            self.url, system, traits
        );
        let res = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", agent.token))
            .send()
            .await?;

        println!("{:?}", res.text().await?);
        Ok(serde_json::Value::Null)
        // let query_waypoint_traits_res: serde_json::Value = res.json().await?;
        // Ok(query_waypoint_traits_res)
        // }
    }

    async fn fetch_systems(&self, agent: &Agent) -> Result<SystemsResponse, anyhow::Error> {
        let url = format!("{}/systems", self.url);
        let res = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", agent.token))
            .send()
            .await?;

        let res: SystemsResponse = res.json().await?;
        Ok(res)
    }
    async fn fetch_ships(&self, agent: &Agent) -> Result<ShipsResponse, anyhow::Error> {
        let url = format!("{}/my/ships", self.url);
        let res = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", agent.token))
            .send()
            .await?;

        let res: ShipsResponse = res.json().await?;
        Ok(res)
    }
}

#[derive(Deserialize, Debug)]
struct ContractsResponse {
    data: Vec<Contract>,
    meta: Meta,
}

#[derive(Deserialize, Serialize, Debug)]
struct Contract {
    id: String,
    #[serde(rename = "factionSymbol")]
    faction_symbol: String,
    #[serde(rename = "type")]
    contract_type: String,
    terms: serde_json::Value, // Terms,
    accepted: bool,
    fulfilled: bool,
    expiration: String,
    #[serde(rename = "deadlineToAccept")]
    deadline_to_accept: String,
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

#[derive(Deserialize, Debug)]
struct SystemsResponse {
    data: Vec<System>,
    meta: Meta,
}

#[derive(Clone, Deserialize, Debug)]
struct System {
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

// {
//       "id": "clreq7vxy5lhbs60cfo6891fa",
//       "factionSymbol": "COSMIC",
//       "type": "PROCUREMENT",
//       "terms": {
//         "deadline": "2024-01-22T09:32:13.540Z",
//         "payment": {
//           "onAccepted": 1247,
//           "onFulfilled": 9378
//         },
//         "deliver": [
//           {
//             "tradeSymbol": "IRON_ORE",
//             "destinationSymbol": "X1-KY62-H52",
//             "unitsRequired": 53,
//             "unitsFulfilled": 0
//           }
//         ]
//       },
//       "accepted": false,
//       "fulfilled": false,
//       "expiration": "2024-01-16T09:32:13.540Z",
//       "deadlineToAccept": "2024-01-16T09:32:13.540Z"
//     }

#[derive(Deserialize, Debug)]
struct Meta {
    total: u32,
    page: u32,
    limit: u32,
}
