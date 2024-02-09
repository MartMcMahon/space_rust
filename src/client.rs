use serde::{Deserialize, Serialize};

use crate::space::{self, Contract, ContractsResponse, SingleContractResponse};

#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct Creds {
    callsign: String,
    token: String,
}

#[derive(Clone)]
pub struct Client {
    creds: Creds,
}
impl Client {
    fn url(path: &str) -> String {
        let base_url = "https://api.spacetraders.io/v2";
        format!("{}{}", base_url, path)
    }
    pub fn new(creds: Creds) -> Client {
        Client { creds }
    }

    pub fn get_user(&self) -> Result<space::Agent, reqwest::Error> {
        let url = Client::url("/my/agent");
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.creds.token))
            .send()
            .unwrap();
        let user = res.json::<space::AgentResponse>().unwrap();
        Ok(user.data)
    }

    pub fn get_contracts(&self) -> Result<Vec<Contract>, reqwest::Error> {
        let url = Client::url("/my/contracts");
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.creds.token))
            .send()
            .unwrap();
        let contracts = res.json::<ContractsResponse>().unwrap();
        Ok(contracts.data)
    }

    pub fn accept_contract(&self, contract_id: &str) -> Result<serde_json::Value, reqwest::Error> {
        println!("accepting contract: {}", contract_id);
        let url = Client::url(("/my/contracts/".to_owned() + contract_id + "/accept").as_str());
        println!("url: {}", url);
        println!("token: {}", self.creds.token);
        let client = reqwest::blocking::Client::new();
        let res = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.creds.token))
            .header("Content-Type", format!("application/json"))
            .body("")
            .send()
            .unwrap();
        println!("{:?}", res);
        // let accept = res.json::<SingleContractResponse>().unwrap();
        // this works, it just throws a 400 status code if contract is already accepted
        Ok(res.json().unwrap())
    }

    pub fn get_systems(&self) -> Result<Vec<space::System>, reqwest::Error> {
        let url = Client::url("/game/systems");
        let client = reqwest::blocking::Client::new();
        let res = client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.creds.token))
            .send()
            .unwrap();
        let systems = res.json::<space::SystemsResponse>().unwrap();
        Ok(systems.data)
    }
}
