use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    debug: bool,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    On,
    Off,
    Get,
}

#[derive(Serialize, Deserialize, Debug)]
struct KeyLightCommand {
    #[serde(rename = "numberOfLights")]
    number_of_lights: u8,
    lights: Vec<KeyLightConfig>,
}

#[derive(Serialize, Deserialize, Debug)]
struct KeyLightConfig {
    on: u8,
}

fn main() {
    let cli = Cli::parse();
    let debug_logs = cli.debug;
    println!("debug enabled: {:?}", debug_logs);
    if debug_logs {
        println!("config file: {:?}", cli.config);
    }

    match &cli.command {
        Some(Commands::On) => {
            println!("Parsed command for on");
            send_command(Commands::On);
        }
        Some(Commands::Off) => {
            println!("Parse command for off");
            send_command(Commands::Off);
        }
        Some(Commands::Get) => {
            println!("Parse command for get");
            send_command(Commands::Get);
        }
        None => {}
    }
}

fn send_command(command: Commands) {
    let client = reqwest::blocking::Client::new();
    match command {
        Commands::On => {
            println!("Sending 'on' command to the light");
            let key_light_config = KeyLightConfig { on: 1 };
            let light_configs = vec![key_light_config];
            let key_light_command = KeyLightCommand {
                number_of_lights: 1,
                lights: light_configs,
            };
            println!("{:?}", &key_light_command);
            let response = client
                .put("http://192.168.1.24:9123/elgato/lights")
                .json(&key_light_command)
                .send();
            let status = response.unwrap().status();
            println!("{}", status);
        }
        Commands::Off => {
            println!("Sending 'off' command to the light");
            let key_light_config = KeyLightConfig { on: 0 };
            let light_configs = vec![key_light_config];
            let key_light_command = KeyLightCommand {
                number_of_lights: 1,
                lights: light_configs,
            };
            println!("{:?}", &key_light_command);
            let response = client
                .put("http://192.168.1.24:9123/elgato/lights")
                .json(&key_light_command)
                .send();
            let status = response.unwrap().status();
            println!("{}", status);
        }
        Commands::Get => {
            println!("Sending 'get' command to the light");
            // Implement the logic to send the 'off' command
            let response = client.get("http://192.168.1.24:9123/elgato/lights").send();
            let status = response.unwrap().status();
            println!("{}", status);
        }
    }
}
