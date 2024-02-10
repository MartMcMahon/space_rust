use libc::STDIN_FILENO;
use std::{
    io::{self, Read, Write},
    thread,
    time::Duration,
};
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};
use tokio::sync::{mpsc, Mutex};

mod client;
mod space;
use client::Client;

#[derive(Default)]
struct AppState {
    credits: i64,
    contracts: Vec<space::Contract>,
    selection_index: usize,
}

fn load_creds() -> String {
    std::fs::read_to_string("auth.json").unwrap()
}

#[tokio::main]
async fn main() {
    let client = start();
}

fn start() -> Client {
    let mut state: Mutex<AppState> = Mutex::new(AppState::default());
    print!("loading credentials from disk");
    let (tx, mut rx) = mpsc::channel(1);

    thread::spawn(move || {
        let creds = load_creds();
        tx.blocking_send(creds).unwrap();
    });

    let client: Client;
    loop {
        match rx.try_recv() {
            Ok(creds) => {
                // println!("creds loaded: {:?}", creds);
                let creds = serde_json::from_str(&creds).unwrap();
                client = Client::new(creds);
                break;
            }
            Err(_) => {}
        }
        print!(".");
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(500));
    }

    print!("\n\nfetching agent data");
    let (tx, mut rx) = mpsc::channel(1);
    let c = client.clone();
    thread::spawn(move || {
        let user = c.get_user().unwrap();
        tx.blocking_send(user.to_string()).unwrap();
    });

    loop {
        match rx.try_recv() {
            Ok(user) => {
                println!("\n\n--- user info ---");
                println!("{}", user);
                break;
            }
            Err(_) => {}
        }
        print!(".");
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(200));
    }

    let mut prompter = Prompter::new();
    loop {
        // println!("name: {}", user.name);
        let nav = main_menu(&mut prompter);
        match nav {
            b'c' => contracts_menu(&mut state, &mut prompter, &client),
            b's' => println!("ships"),
            b'y' => println!("systems"),
            b'q' => break,
            27 => break,
            _ => println!("Invalid input"),
        }
    }

    client
}

fn clear_screen() {
    println!("\x1b[2J");
}

fn contracts_menu(state: &mut Mutex<AppState>, prompter: &mut Prompter, client: &Client) {
    let (tx, mut rx) = mpsc::channel(1);
    let c = client.clone();
    thread::spawn(move || {
        let contracts = c.get_contracts().unwrap();
        tx.blocking_send(contracts).unwrap();
    });

    loop {
        match rx.try_recv() {
            Ok(contracts) => {
                state.get_mut().contracts = contracts;
                break;
            }
            Err(_) => {}
        }
        print!(".");
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(200));
    }

    let state = state.get_mut();
    state.contracts.push(space::Contract {
        id: "123".to_string(),
        contract_type: "test".to_string(),
        faction_symbol: "test".to_string(),
        accepted: false,
        fulfilled: false,
        expiration: "test".to_string(),
        deadline_to_accept: "test".to_string(),
        terms: serde_json::Value::Null,
    });
    loop {
        println!("\n=== contracts ({}) ===", state.contracts.len());
        state.contracts.iter().enumerate().for_each(|(i, c)| {
            if i == state.selection_index {
                print!(
                    "
===============
| faction: {}
| type: {}
| accepted: {}
| fulfilled: {}
| expiration: {}
| deadline to accept: {}
===============
",
                    c.faction_symbol,
                    c.contract_type,
                    c.accepted,
                    c.fulfilled,
                    c.expiration,
                    c.deadline_to_accept
                );
            } else {
                print!("id: {}", c.id);
            }
        });

        // let s = format!("\n=== contracts ({}) ===", num);

        let nav = prompter.read_menu_input("");
        match nav {
            MenuInput::Up => {
                if state.selection_index > 0 {
                    state.selection_index -= 1;
                }
            }
            MenuInput::Down => {
                if state.selection_index < state.contracts.len() - 1 {
                    state.selection_index += 1;
                }
            }
            MenuInput::Enter => {
                let mut prompter = Prompter::new();
                single_contract_menu(
                    &state.contracts[state.selection_index],
                    client,
                    &mut prompter,
                );
            }
            MenuInput::Escape => {
                break;
            }
            _ => {}
        }
    }
}

fn single_contract_menu(contract: &space::Contract, client: &Client, prompter: &mut Prompter) {
    let contract = contract.clone();
    let nav = prompter.read_single_char("\naccept? (y/n)").unwrap();
    match nav {
        b'y' => {
            let (tx, mut rx) = mpsc::channel(1);
            let client = client.clone();
            thread::spawn(move || {
                let accept_contract_res = client.accept_contract(&contract.id.clone());
                println!("{:?}", accept_contract_res);
                tx.blocking_send(accept_contract_res).unwrap();
            });

            loop {
                match rx.try_recv() {
                    Ok(accept_contract_res) => {
                        println!("accepted");
                        break;
                    }
                    Err(_) => {}
                }
                print!(".");
                std::io::stdout().flush().unwrap();
                thread::sleep(Duration::from_millis(200));
            }
        }
        b'n' => {}
        _ => println!("Invalid input"),
    }
}

fn systems_menu(client: Client, prompter: &mut Prompter) {
    let (tx, mut rx) = mpsc::channel(1);
    let client = client.clone();
    thread::spawn(move || {
        let systems = client.get_systems().unwrap();
        tx.blocking_send(systems).unwrap();
    });

    loop {
        match rx.try_recv() {
            Ok(systems) => {
                println!("systems: {:?}", systems);
                break;
            }
            Err(_) => {}
        }
        print!(".");
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(200));
    }
}

fn ships_menu(prompter: &mut Prompter) {
    let s = "=== ships ===";
    // loop through ships
    // l -- list ships
    // q -- quit
    //  >"#;

    let nav = prompter.read_single_char(s).unwrap();
    match nav {
        b'l' => {}
        b'q' => {}
        _ => println!("Invalid input"),
    }
}

fn main_menu(prompter: &mut Prompter) -> u8 {
    let s = r#"
|==== main menu ====|
c -- contracts
s -- ships
y -- systems
q -- quit

 >"#;
    return prompter.read_single_char(s).unwrap();
}

struct Prompter {
    stdin: i32,
    stdout: io::Stdout,
    reader: io::Stdin,
    buffer: [u8; 1],
}
impl Prompter {
    fn new() -> Prompter {
        let stdin = STDIN_FILENO;
        let termios = Termios::from_fd(stdin).unwrap();
        let mut new_termios = termios.clone(); // make a mutable copy of termios
        let stdout = io::stdout();
        let reader = io::stdin();
        let buffer = [0; 1];

        new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
        tcsetattr(stdin, TCSANOW, &mut new_termios).unwrap();

        Prompter {
            stdin,
            stdout,
            reader,
            buffer,
        }
    }
    fn read_single_char(&mut self, msg: &str) -> io::Result<u8> {
        print!("{}", msg);
        self.stdout.lock().flush().unwrap();
        self.reader.read_exact(&mut self.buffer).unwrap();
        // tcsetattr(stdin, TCSANOW, &termios).unwrap();
        Ok(self.buffer[0])
    }

    fn read_menu_input(&mut self, msg: &str) -> MenuInput {
        let mut buffer = [0; 3];
        print!("{}", msg);
        self.stdout.lock().flush().unwrap();
        self.reader.read(&mut buffer).unwrap();
        match buffer[0] {
            b'k' => MenuInput::Up,
            b'j' => MenuInput::Down,
            b'h' => MenuInput::Left,
            b'l' => MenuInput::Right,
            10 => MenuInput::Enter,
            27 => {
                if buffer[1] == 91 {
                    return match buffer[2] {
                        65 => MenuInput::Up,
                        66 => MenuInput::Down,
                        67 => MenuInput::Right,
                        68 => MenuInput::Left,
                        _ => MenuInput::Escape,
                    };
                } else {
                    return MenuInput::Escape;
                }
            }
            _ => MenuInput::Escape,
        }
    }
}

enum MenuInput {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Escape,
}

enum ContractMenuResult {
    Accept,
    Quit,
}
