mod structs;

use std::fs::{File, read_to_string};
use std::io::Write;
use tracing_subscriber::FmtSubscriber;

use clap::{Parser, Subcommand};
use clap_serde_derive::{
    clap::{self},
    serde::Serialize
};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::info;
use crate::structs::commands::{Cli, Commands};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Packages{
    name: String,
    version: String,
    excepted: bool
}

impl Packages{
    fn new(name: String, version: String) -> Self{
        Packages{
            name,
            version,
            excepted: false,
        }
    }
}

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(FmtSubscriber::default()).unwrap();

    let cli = Cli::parse();

    if cli.command.is_some(){
        let cli = cli.command.unwrap();

        match cli {
            Commands::Init => {
                let mut file = File::create("checker.json").unwrap();

                let package = Packages::new(String::from("@blocknote/core"),String::from("^0.12.1"));
                let test = json!(package);
                let test = serde_json::to_string_pretty(&test).unwrap();
                file.write_all(test.as_ref()).unwrap();

                let contents = read_to_string("package.json").ok().unwrap();
                let v = serde_json::from_str::<Value>(contents.as_str()).ok().unwrap();

                let dependencies = v.get("dependencies").unwrap().as_object().unwrap();
                dependencies.iter().for_each(|(package_name, version)|{

                    info!("package_name : {:?}", package_name);
                    info!("version : {:?}", version.as_str().unwrap());

                });
            }
        }
    }
}