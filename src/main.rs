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

// #[derive(Deserialize, Debug)]
// struct QueryWaypointTraitsResponse {
//     data: Vec<serde_json::Value>,
//     meta: Meta,
// }
//
//
//
//
//
//
//
//

// {
//   "data": [
//     {
//       "symbol": "X1-DB72",
//       "sectorSymbol": "X1",
//       "type": "BLUE_STAR",
//       "x": 1099,
//       "y": 2130,
//       "waypoints": [
//         {
//           "symbol": "X1-DB72-ZZ1C",
//           "type": "PLANET",
//           "x": 2,
//           "y": 13,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-DB72-XC8X",
//           "type": "ASTEROID",
//           "x": 550,
//           "y": 475,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-DB72-Z10E",
//           "type": "ASTEROID",
//           "x": -630,
//           "y": 343,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-DB72-BA2C",
//           "type": "ASTEROID",
//           "x": -164,
//           "y": 714,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-DB72-ZB7X",
//           "type": "ASTEROID",
//           "x": 703,
//           "y": -141,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-DB72-CB3X",
//           "type": "ASTEROID",
//           "x": -342,
//           "y": 647,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-DB72-AX9A",
//           "type": "ASTEROID",
//           "x": 464,
//           "y": 557,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-DB72-XF4E",
//           "type": "ASTEROID",
//           "x": -567,
//           "y": -526,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-DB72-ZB6C",
//           "type": "ASTEROID",
//           "x": 323,
//           "y": -637,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-DB72-FA5F",
//           "type": "ASTEROID",
//           "x": -250,
//           "y": -723,
//           "orbitals": []
//         }
//       ],
//       "factions": []
//     },
//     {
//       "symbol": "X1-TF2",
//       "sectorSymbol": "X1",
//       "type": "BLACK_HOLE",
//       "x": 252,
//       "y": -1783,
//       "waypoints": [],
//       "factions": []
//     },
//     {
//       "symbol": "X1-TZ10",
//       "sectorSymbol": "X1",
//       "type": "BLUE_STAR",
//       "x": -579,
//       "y": 3087,
//       "waypoints": [
//         {
//           "symbol": "X1-TZ10-FF4E",
//           "type": "ASTEROID",
//           "x": 202,
//           "y": 750,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TZ10-CB1Z",
//           "type": "PLANET",
//           "x": 5,
//           "y": -10,
//           "orbitals": [
//             {
//               "symbol": "X1-TZ10-BE2X"
//             }
//           ]
//         },
//         {
//           "symbol": "X1-TZ10-BE6D",
//           "type": "ASTEROID",
//           "x": -720,
//           "y": 139,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TZ10-CZ5C",
//           "type": "ASTEROID",
//           "x": -295,
//           "y": 699,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TZ10-FC3B",
//           "type": "ASTEROID",
//           "x": 784,
//           "y": -38,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TZ10-X12D",
//           "type": "ASTEROID",
//           "x": 586,
//           "y": -401,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TZ10-A11D",
//           "type": "ASTEROID",
//           "x": -269,
//           "y": -666,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TZ10-AC7C",
//           "type": "ASTEROID",
//           "x": -583,
//           "y": -492,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TZ10-A13C",
//           "type": "ASTEROID",
//           "x": 661,
//           "y": 328,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TZ10-EA8C",
//           "type": "ASTEROID",
//           "x": -656,
//           "y": -346,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TZ10-AZ9Z",
//           "type": "ASTEROID",
//           "x": -532,
//           "y": -565,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TZ10-F10A",
//           "type": "ASTEROID",
//           "x": -524,
//           "y": -569,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TZ10-BE2X",
//           "type": "MOON",
//           "x": 5,
//           "y": -10,
//           "orbitals": [],
//           "orbits": "X1-TZ10-CB1Z"
//         }
//       ],
//       "factions": []
//     },
//     {
//       "symbol": "X1-RA30",
//       "sectorSymbol": "X1",
//       "type": "YOUNG_STAR",
//       "x": -6086,
//       "y": 2,
//       "waypoints": [
//         {
//           "symbol": "X1-RA30-AB2E",
//           "type": "ASTEROID",
//           "x": -747,
//           "y": 120,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RA30-ZC1C",
//           "type": "GAS_GIANT",
//           "x": 7,
//           "y": 8,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RA30-B10Z",
//           "type": "ASTEROID",
//           "x": 259,
//           "y": 692,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RA30-CX6Z",
//           "type": "ASTEROID",
//           "x": 603,
//           "y": -475,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RA30-BC3X",
//           "type": "ASTEROID",
//           "x": -407,
//           "y": -669,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RA30-AE4F",
//           "type": "ASTEROID",
//           "x": -567,
//           "y": -465,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RA30-XA7E",
//           "type": "ASTEROID",
//           "x": 518,
//           "y": -564,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RA30-F11D",
//           "type": "ASTEROID",
//           "x": -404,
//           "y": 593,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RA30-CE8A",
//           "type": "ASTEROID",
//           "x": 690,
//           "y": -283,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RA30-EE5B",
//           "type": "ASTEROID",
//           "x": -107,
//           "y": -760,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RA30-BA9E",
//           "type": "ASTEROID",
//           "x": 529,
//           "y": 501,
//           "orbitals": []
//         }
//       ],
//       "factions": []
//     },
//     {
//       "symbol": "X1-UM18",
//       "sectorSymbol": "X1",
//       "type": "YOUNG_STAR",
//       "x": -1606,
//       "y": 1241,
//       "waypoints": [
//         {
//           "symbol": "X1-UM18-EZ3D",
//           "type": "PLANET",
//           "x": 23,
//           "y": -20,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-UM18-X10F",
//           "type": "ASTEROID",
//           "x": -232,
//           "y": 746,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-UM18-BD4C",
//           "type": "GAS_GIANT",
//           "x": -20,
//           "y": -44,
//           "orbitals": [
//             {
//               "symbol": "X1-UM18-AC5Z"
//             }
//           ]
//         },
//         {
//           "symbol": "X1-UM18-BB1D",
//           "type": "PLANET",
//           "x": 16,
//           "y": 11,
//           "orbitals": [
//             {
//               "symbol": "X1-UM18-BA2D"
//             }
//           ]
//         },
//         {
//           "symbol": "X1-UM18-F11X",
//           "type": "ASTEROID",
//           "x": -722,
//           "y": -221,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-UM18-C12C",
//           "type": "ASTEROID",
//           "x": 246,
//           "y": -688,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-UM18-ZE6E",
//           "type": "ASTEROID",
//           "x": 118,
//           "y": -708,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-UM18-AB7D",
//           "type": "ASTEROID",
//           "x": -150,
//           "y": -756,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-UM18-DX8B",
//           "type": "ASTEROID",
//           "x": 743,
//           "y": 29,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-UM18-XE9B",
//           "type": "ASTEROID",
//           "x": 341,
//           "y": 632,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-UM18-AC5Z",
//           "type": "MOON",
//           "x": -20,
//           "y": -44,
//           "orbitals": [],
//           "orbits": "X1-UM18-BD4C"
//         },
//         {
//           "symbol": "X1-UM18-BA2D",
//           "type": "MOON",
//           "x": 16,
//           "y": 11,
//           "orbitals": [],
//           "orbits": "X1-UM18-BB1D"
//         }
//       ],
//       "factions": []
//     },
//     {
//       "symbol": "X1-RJ25",
//       "sectorSymbol": "X1",
//       "type": "ORANGE_STAR",
//       "x": 2494,
//       "y": 1273,
//       "waypoints": [
//         {
//           "symbol": "X1-RJ25-FF1Z",
//           "type": "ASTEROID",
//           "x": 358,
//           "y": 663,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RJ25-XD4Z",
//           "type": "ASTEROID",
//           "x": -728,
//           "y": -24,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RJ25-XD6F",
//           "type": "ASTEROID",
//           "x": 143,
//           "y": -703,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RJ25-XA5C",
//           "type": "ASTEROID",
//           "x": -610,
//           "y": -497,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RJ25-FB9X",
//           "type": "ASTEROID",
//           "x": 410,
//           "y": -669,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RJ25-FX2A",
//           "type": "ASTEROID",
//           "x": 19,
//           "y": 724,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RJ25-DE7C",
//           "type": "ASTEROID",
//           "x": 406,
//           "y": -665,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RJ25-Z10E",
//           "type": "ASTEROID",
//           "x": 627,
//           "y": 345,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RJ25-DB3X",
//           "type": "ASTEROID",
//           "x": -750,
//           "y": 104,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-RJ25-ZA8A",
//           "type": "ASTEROID",
//           "x": -60,
//           "y": -747,
//           "orbitals": []
//         }
//       ],
//       "factions": []
//     },
//     {
//       "symbol": "X1-GJ20",
//       "sectorSymbol": "X1",
//       "type": "RED_STAR",
//       "x": -3463,
//       "y": -4810,
//       "waypoints": [
//         {
//           "symbol": "X1-GJ20-AE1B",
//           "type": "PLANET",
//           "x": 5,
//           "y": 14,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-GJ20-DB2D",
//           "type": "PLANET",
//           "x": -4,
//           "y": -32,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-GJ20-EE9B",
//           "type": "ASTEROID",
//           "x": -313,
//           "y": -701,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-GJ20-DE3A",
//           "type": "PLANET",
//           "x": 16,
//           "y": -42,
//           "orbitals": [
//             {
//               "symbol": "X1-GJ20-CA4C"
//             }
//           ]
//         },
//         {
//           "symbol": "X1-GJ20-B12F",
//           "type": "ASTEROID",
//           "x": 417,
//           "y": 661,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-GJ20-FF5E",
//           "type": "ASTEROID",
//           "x": 195,
//           "y": 692,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-GJ20-BX6E",
//           "type": "ASTEROID",
//           "x": -682,
//           "y": 323,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-GJ20-B11C",
//           "type": "ASTEROID",
//           "x": 721,
//           "y": -60,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-GJ20-AX7A",
//           "type": "ASTEROID",
//           "x": -498,
//           "y": -559,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-GJ20-Z10F",
//           "type": "ASTEROID",
//           "x": 349,
//           "y": -701,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-GJ20-XA8D",
//           "type": "ASTEROID",
//           "x": -28,
//           "y": -747,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-GJ20-CA4C",
//           "type": "MOON",
//           "x": 16,
//           "y": -42,
//           "orbitals": [],
//           "orbits": "X1-GJ20-DE3A"
//         }
//       ],
//       "factions": []
//     },
//     {
//       "symbol": "X1-TV86",
//       "sectorSymbol": "X1",
//       "type": "RED_STAR",
//       "x": 1534,
//       "y": -2782,
//       "waypoints": [
//         {
//           "symbol": "X1-TV86-D39E",
//           "type": "ASTEROID",
//           "x": -614,
//           "y": 417,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-A11D",
//           "type": "ASTEROID",
//           "x": 304,
//           "y": -176,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-B66E",
//           "type": "ASTEROID",
//           "x": 566,
//           "y": 484,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-A18D",
//           "type": "ASTEROID",
//           "x": -211,
//           "y": 245,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-D25A",
//           "type": "ASTEROID",
//           "x": 588,
//           "y": 525,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-DC7X",
//           "type": "ASTEROID",
//           "x": 257,
//           "y": -250,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-A62D",
//           "type": "ASTEROID",
//           "x": 720,
//           "y": 268,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-E49F",
//           "type": "ASTEROID",
//           "x": -276,
//           "y": -728,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-D28D",
//           "type": "ASTEROID",
//           "x": -237,
//           "y": 742,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-EX2A",
//           "type": "ASTEROID",
//           "x": -262,
//           "y": -270,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-F13E",
//           "type": "ASTEROID",
//           "x": 311,
//           "y": 66,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-C14B",
//           "type": "ASTEROID",
//           "x": 200,
//           "y": 327,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-B20Z",
//           "type": "ASTEROID",
//           "x": -314,
//           "y": 31,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-XB3F",
//           "type": "ASTEROID",
//           "x": -345,
//           "y": -164,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-D12X",
//           "type": "ASTEROID",
//           "x": 282,
//           "y": 145,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-B48Z",
//           "type": "ASTEROID",
//           "x": -364,
//           "y": -636,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-D54Z",
//           "type": "ASTEROID",
//           "x": 530,
//           "y": -554,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-A27E",
//           "type": "ASTEROID",
//           "x": 338,
//           "y": 683,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-X37C",
//           "type": "ASTEROID",
//           "x": -726,
//           "y": 127,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-Z64X",
//           "type": "ASTEROID",
//           "x": 754,
//           "y": 13,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-X17D",
//           "type": "ASTEROID",
//           "x": -195,
//           "y": 323,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-E22C",
//           "type": "JUMP_GATE",
//           "x": 314,
//           "y": -325,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-ZD6E",
//           "type": "ASTEROID",
//           "x": -82,
//           "y": -375,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-B43C",
//           "type": "ASTEROID",
//           "x": -619,
//           "y": -461,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-B16E",
//           "type": "ASTEROID",
//           "x": -246,
//           "y": 237,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-B30A",
//           "type": "ASTEROID",
//           "x": -484,
//           "y": 562,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-Z36C",
//           "type": "ASTEROID",
//           "x": -630,
//           "y": 431,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-E38Z",
//           "type": "ASTEROID",
//           "x": -607,
//           "y": 479,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-C45Z",
//           "type": "ASTEROID",
//           "x": -755,
//           "y": -114,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-B65Z",
//           "type": "ASTEROID",
//           "x": 339,
//           "y": 716,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-D52F",
//           "type": "ASTEROID",
//           "x": 154,
//           "y": -765,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-Z24E",
//           "type": "ASTEROID",
//           "x": 272,
//           "y": 670,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-F32C",
//           "type": "ASTEROID",
//           "x": -652,
//           "y": 383,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-XE5C",
//           "type": "ASTEROID",
//           "x": 86,
//           "y": -378,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-B44X",
//           "type": "ASTEROID",
//           "x": -694,
//           "y": -318,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-E46F",
//           "type": "ASTEROID",
//           "x": -693,
//           "y": -289,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-D21B",
//           "type": "ASTEROID",
//           "x": -335,
//           "y": 17,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-A26F",
//           "type": "ASTEROID",
//           "x": 526,
//           "y": 539,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-ZD4B",
//           "type": "ASTEROID",
//           "x": -35,
//           "y": -328,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-Z61B",
//           "type": "ASTEROID",
//           "x": 730,
//           "y": 56,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-Z15C",
//           "type": "ASTEROID",
//           "x": 18,
//           "y": 362,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-X23E",
//           "type": "ASTEROID",
//           "x": 348,
//           "y": 667,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-E34F",
//           "type": "ASTEROID",
//           "x": -787,
//           "y": -19,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-DA1Z",
//           "type": "ASTEROID",
//           "x": -347,
//           "y": -100,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-C63F",
//           "type": "ASTEROID",
//           "x": 559,
//           "y": 469,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-X50E",
//           "type": "ASTEROID",
//           "x": -132,
//           "y": -696,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-F53C",
//           "type": "ASTEROID",
//           "x": 622,
//           "y": -396,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-A19E",
//           "type": "ASTEROID",
//           "x": -273,
//           "y": 236,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-EF8Z",
//           "type": "ASTEROID",
//           "x": 20,
//           "y": -356,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-X59Z",
//           "type": "ASTEROID",
//           "x": 689,
//           "y": 299,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-C51A",
//           "type": "ASTEROID",
//           "x": 290,
//           "y": -716,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-X29X",
//           "type": "ASTEROID",
//           "x": 146,
//           "y": 724,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-B33Z",
//           "type": "ASTEROID",
//           "x": -213,
//           "y": 736,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-D40C",
//           "type": "ASTEROID",
//           "x": -736,
//           "y": 32,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-A60A",
//           "type": "ASTEROID",
//           "x": 666,
//           "y": 331,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-D47C",
//           "type": "ASTEROID",
//           "x": -519,
//           "y": -515,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-C31B",
//           "type": "ASTEROID",
//           "x": -662,
//           "y": 386,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-X35D",
//           "type": "ASTEROID",
//           "x": -561,
//           "y": 512,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-E41A",
//           "type": "ASTEROID",
//           "x": -691,
//           "y": -161,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-E56C",
//           "type": "ASTEROID",
//           "x": 581,
//           "y": -505,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-B55F",
//           "type": "ASTEROID",
//           "x": 742,
//           "y": -101,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-D57X",
//           "type": "ASTEROID",
//           "x": 436,
//           "y": -583,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-D42B",
//           "type": "ASTEROID",
//           "x": -700,
//           "y": -312,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-F58C",
//           "type": "ASTEROID",
//           "x": 704,
//           "y": 118,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-BD9A",
//           "type": "ASTEROID",
//           "x": 340,
//           "y": -156,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-TV86-D10B",
//           "type": "ASTEROID",
//           "x": 345,
//           "y": -76,
//           "orbitals": []
//         }
//       ],
//       "factions": []
//     },
//     {
//       "symbol": "X1-YV4",
//       "sectorSymbol": "X1",
//       "type": "RED_STAR",
//       "x": -2214,
//       "y": 1285,
//       "waypoints": [
//         {
//           "symbol": "X1-YV4-DB7D",
//           "type": "ASTEROID",
//           "x": -646,
//           "y": 379,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-YV4-ZB5X",
//           "type": "GAS_GIANT",
//           "x": 82,
//           "y": 25,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-YV4-AD9Z",
//           "type": "ASTEROID",
//           "x": -676,
//           "y": 354,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-YV4-X12X",
//           "type": "ASTEROID",
//           "x": 4,
//           "y": -758,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-YV4-AZ6A",
//           "type": "ASTEROID",
//           "x": 186,
//           "y": 749,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-YV4-EX8Z",
//           "type": "ASTEROID",
//           "x": -417,
//           "y": 613,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-YV4-E13X",
//           "type": "ASTEROID",
//           "x": 93,
//           "y": -741,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-YV4-AC3A",
//           "type": "PLANET",
//           "x": 35,
//           "y": 15,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-YV4-EZ4F",
//           "type": "GAS_GIANT",
//           "x": 23,
//           "y": -56,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-YV4-F14A",
//           "type": "ASTEROID",
//           "x": 700,
//           "y": -339,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-YV4-D11C",
//           "type": "ASTEROID",
//           "x": -712,
//           "y": -315,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-YV4-X10X",
//           "type": "ASTEROID",
//           "x": -681,
//           "y": -390,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-YV4-C15X",
//           "type": "ASTEROID",
//           "x": 671,
//           "y": 244,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-YV4-BX1B",
//           "type": "PLANET",
//           "x": -5,
//           "y": -9,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-YV4-BC2C",
//           "type": "PLANET",
//           "x": -20,
//           "y": 17,
//           "orbitals": []
//         }
//       ],
//       "factions": []
//     },
//     {
//       "symbol": "X1-HY85",
//       "sectorSymbol": "X1",
//       "type": "BLUE_STAR",
//       "x": -5918,
//       "y": -3688,
//       "waypoints": [
//         {
//           "symbol": "X1-HY85-A17X",
//           "type": "ASTEROID",
//           "x": 770,
//           "y": -133,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-EE3F",
//           "type": "MOON",
//           "x": 17,
//           "y": -13,
//           "orbitals": [],
//           "orbits": "X1-HY85-BZ2B"
//         },
//         {
//           "symbol": "X1-HY85-B16F",
//           "type": "ASTEROID",
//           "x": 48,
//           "y": -721,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-CD4C",
//           "type": "MOON",
//           "x": 17,
//           "y": -13,
//           "orbitals": [],
//           "orbits": "X1-HY85-BZ2B"
//         },
//         {
//           "symbol": "X1-HY85-B19F",
//           "type": "ASTEROID",
//           "x": 651,
//           "y": 395,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-F20B",
//           "type": "ASTEROID",
//           "x": -265,
//           "y": 741,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-CC7Z",
//           "type": "ASTEROID",
//           "x": -690,
//           "y": 231,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-F14A",
//           "type": "ASTEROID",
//           "x": -567,
//           "y": -496,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-A21C",
//           "type": "ASTEROID",
//           "x": -319,
//           "y": 712,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-ZX8E",
//           "type": "ASTEROID",
//           "x": -647,
//           "y": 386,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-E15E",
//           "type": "ASTEROID",
//           "x": -69,
//           "y": -743,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-BZ2B",
//           "type": "PLANET",
//           "x": 17,
//           "y": -13,
//           "orbitals": [
//             {
//               "symbol": "X1-HY85-EE3F"
//             },
//             {
//               "symbol": "X1-HY85-CD4C"
//             }
//           ]
//         },
//         {
//           "symbol": "X1-HY85-BD5A",
//           "type": "ASTEROID",
//           "x": 73,
//           "y": 727,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-CD1F",
//           "type": "PLANET",
//           "x": 11,
//           "y": 2,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-D10C",
//           "type": "ASTEROID",
//           "x": -695,
//           "y": 219,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-CB9E",
//           "type": "ASTEROID",
//           "x": -627,
//           "y": -339,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-CA6A",
//           "type": "ASTEROID",
//           "x": -454,
//           "y": 589,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-X11D",
//           "type": "ASTEROID",
//           "x": -744,
//           "y": 31,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-F12Z",
//           "type": "ASTEROID",
//           "x": -726,
//           "y": 209,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-Z13C",
//           "type": "ASTEROID",
//           "x": -703,
//           "y": -259,
//           "orbitals": []
//         },
//         {
//           "symbol": "X1-HY85-D18F",
//           "type": "ASTEROID",
//           "x": 648,
//           "y": -350,
//           "orbitals": []
//         }
//       ],
//       "factions": []
//     }
//   ],
//   "meta": {
//     "total": 8498,
//     "page": 1,
//     "limit": 10
//   }
// }
