use std::fs::File;
use std::io::Read;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Parser, Debug)]
enum Command {
    Register {
        #[arg(short, long)]
        callsign: String,
    },
}

fn main() {
    let args = Args::parse();

    // let mut buffer = String::new();
    // io::stdin().read_line(&mut buffer)?;

    match args.command {
        Command::Register { callsign } => {
            println!("Registering {}", callsign);
        }
        _ => println!("No command"),
    }
}

// fn read_auth() {
//     read_auth_file("auth.json".to_string())
// }

fn read_auth_file(filename: String) -> Result<String, std::io::Error> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(serde_json::from_str(&contents)?)
}

struct Creds {
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
    async fn list_contracts(&self) -> Result<String, reqwest::Error> {
        let url = format!("{}/my/contracts", self.url);
        let res = self.client.get(&url).send().await?;
        Ok(res.text().await?)
    }
}

// .then(response => response.json())
//   .then(response => console.log(response))
//   .catch(err => console.error(err));
