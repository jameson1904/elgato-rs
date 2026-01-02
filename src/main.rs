use std::path::PathBuf;

use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};

use futures_util::{pin_mut, stream::StreamExt};
use mdns::{Error, Record, RecordKind};
use std::{net::IpAddr, time::Duration};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    debug: bool,

    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    On,
    Off,
    Get,
    Discover,
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

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let debug_logs = cli.debug;
    println!("debug enabled: {:?}", debug_logs);
    if debug_logs {
        println!("config file: {:?}", cli.config);
    }

    match &cli.command {
        Some(Command::On) => {
            println!("Parsed command for on");
            handle_command(Command::On).await;
        }
        Some(Command::Off) => {
            println!("Parse command for off");
            handle_command(Command::Off).await;
        }
        Some(Command::Get) => {
            println!("Parse command for get");
            handle_command(Command::Get).await;
        }
        Some(Command::Discover) => {
            println!("Parse command for discover");
            handle_command(Command::Discover).await;
        }
        None => {}
    }
}

async fn handle_command(command: Command) {
    //let client = reqwest::blocking::Client::new();
    let client = reqwest::Client::new();
    match command {
        Command::On => {
            println!("Sending 'on' command to the light");
            let key_light_config = KeyLightConfig { on: 1 };
            let light_configs = vec![key_light_config];
            let key_light_command = KeyLightCommand {
                number_of_lights: 1,
                lights: light_configs,
            };
            println!("{:?}", &key_light_command);
            let _response = client
                .put("http://192.168.1.24:9123/elgato/lights")
                .json(&key_light_command)
                .send()
                .await;
            //let status = response.unwrap().status();
            //println!("{}", status);
        }
        Command::Off => {
            println!("Sending 'off' command to the light");
            let key_light_config = KeyLightConfig { on: 0 };
            let light_configs = vec![key_light_config];
            let key_light_command = KeyLightCommand {
                number_of_lights: 1,
                lights: light_configs,
            };
            println!("{:?}", &key_light_command);
            let _response = client
                .put("http://192.168.1.24:9123/elgato/lights")
                .json(&key_light_command)
                .send()
                .await;
            //let status = response.unwrap().status();
            //println!("{}", status);
        }
        Command::Get => {
            println!("Sending 'get' command to the light");
            // Implement the logic to send the 'off' command
            let _response = client.get("http://192.168.1.24:9123/elgato/lights").send();
            //let status = response.unwrap().status();
            //println!("{}", status);
        }
        Command::Discover => {
            let _ = discover_devices().await;
        }
    }
}

async fn discover_devices() -> Result<(), Error> {
    println!("Discovering Elgato Devices");
    const SERVICE_NAME: &'static str = "_elg._tcp.local";
    let stream = mdns::discover::all(SERVICE_NAME, Duration::from_secs(1))?.listen();
    pin_mut!(stream);

    while let Some(Ok(response)) = stream.next().await {
        let addr = response.records().filter_map(self::to_ip_addr).next();

        if let Some(addr) = addr {
            println!("found cast device at {}", addr);
        } else {
            println!("cast device does not advertise address");
        }
    }

    Ok(())
}

fn to_ip_addr(record: &Record) -> Option<IpAddr> {
    match record.kind {
        RecordKind::A(addr) => Some(addr.into()),
        RecordKind::AAAA(addr) => Some(addr.into()),
        _ => None,
    }
}
