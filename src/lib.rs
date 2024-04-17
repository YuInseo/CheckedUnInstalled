
use std::fs::read_to_string;
use std::path::Path;
use clap::Command;
use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::presets::UTF8_FULL;
use comfy_table::{Cell, CellAlignment, ContentArrangement, Table};
use serde_json::{Map, Value};
use tracing::info;

pub fn get_map_from_json(path: &str) -> Result<Map<String, Value>, (&str)>{
    let is_existed = Path::new(path).is_file();

    if is_existed{
        let package_json = read_to_string(path).ok().unwrap();
        let value = serde_json::from_str::<Value>(package_json.as_str()).ok().unwrap();
        let value = value.as_object().unwrap();
        let dependencies = value.get("dependencies").unwrap().as_object().unwrap();

        Ok(dependencies.to_owned())
    } else{
        Err("package.json is not existed on this folder")
    }
}

pub fn showing_as_table(target_map: Map<String, Value>, current_map: Map<String, Value>){


    info!("Press Enter to install");

    let mut new_vec: Vec<String> = Vec::new();
    target_map.iter().for_each(|(name, value)|{
        if new_vec.len() <= 10{
            new_vec.push(name.to_owned());

            info!("test : {:?}", name);
        }
    });


    //TODO Think about table algorithm & page algorithm

    info!("Press Right Arrow Key to show the next page")
}

pub fn get_origin_map() -> Map<String, Value>{
    let map: Map<String,Value> = Map::new();

    let contents = read_to_string("checker.json").ok().unwrap();
    let value = serde_json::from_str::<Value>(contents.as_str()).ok().unwrap();

    let object = value.as_object().unwrap();
    let path = object.get("path").unwrap().as_str().unwrap();

    let is_folder = Path::new(path).is_dir();
    if is_folder {
        let package_json_path = &(path.to_owned() + "package.json");

        return get_map_from_json(package_json_path).ok().unwrap()
    }

    map
}