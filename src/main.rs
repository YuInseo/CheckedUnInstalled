mod structs;

use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::Path;
use tracing_subscriber::FmtSubscriber;

use clap::{Parser, Subcommand};
use clap_serde_derive::{
    clap::{self},
    serde::Serialize
};
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::info;
use CheckUninstalled::get_map_from_json;
use crate::structs::commands::{Cli, Commands};
use crate::structs::option::PackageOption;
use crate::structs::package::Packages;

#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(FmtSubscriber::default()).unwrap();

    let cli = Cli::parse();

    if cli.command.is_some(){
        let cli = cli.command.unwrap();

        match cli {
            Commands::Init => {
                let is_existed = Path::new("checker.json").is_file();

                if is_existed{
                    let contents = read_to_string("checker.json").ok().unwrap();
                    let value = serde_json::from_str::<Value>(contents.as_str()).ok().unwrap();

                    let object = value.as_object().unwrap();
                    let path = object.get("path").unwrap().as_str().unwrap();

                    let is_folder = Path::new(path).is_dir();
                    if is_folder{
                        let package_json_path = &(path.to_owned() + "package.json");

                        let target_map = get_map_from_json(package_json_path).ok().unwrap();
                        let current_map = get_map_from_json("package.json").ok().unwrap();
                        let all_keys: Vec<_> = target_map.keys().chain(current_map.keys()).collect();

                        let differences: Vec<_> = all_keys.into_iter()
                            .filter(|key| target_map.get(key.as_str()) != current_map.get::<str>(key.as_ref()))
                            .map(|key| key.clone())
                            .collect();

                        differences.iter().for_each(|different|{
                            let mut install_command: String = String::new();

                            install_command += &*(String::from("npm i ") + different);

                            info!("test {:?}", install_command);

                        });
                    } else{

                    }
                } else{
                    let mut file = File::create("checker.json").ok();
                    let package_option = PackageOption::new();

                    let test = json!(package_option);
                    let test = serde_json::to_string_pretty(&test).unwrap();
                    file.unwrap().write_all(test.as_ref()).ok().unwrap();
                }
            }
        }
    }
}