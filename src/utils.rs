
use std::fs::{OpenOptions, read_to_string};
use std::io::Write;
use std::path::Path;
use serde_json::{json, Map, Value};
use tracing::info;
use crate::structs::import_option::{ImportOption, ImportOptionBuilder};

pub fn get_map_from_json(path: &str) -> Result<Map<String, Value>, String>{
    let is_existed = Path::new(path).is_file();

    if is_existed{
        let package_json = read_to_string(path).ok().unwrap();
        let value = serde_json::from_str::<Value>(package_json.as_str()).ok().unwrap();
        let value = value.as_object().unwrap();
        let dependencies = value.get("dependencies").unwrap().as_object().unwrap();

        Ok(dependencies.to_owned())
    } else{
        Err(String::from("package.json is not existed on this folder"))
    }
}

pub fn showing_as_table(target_map: Map<String, Value>, current_map: Map<String, Value>){


    info!("Press Enter to install");

    let mut new_vec: Vec<String> = Vec::new();
    target_map.iter().for_each(|(name, value)|{
        if new_vec.len() <= 10{
            new_vec.push(name.to_owned());
            ImportOption::new();

            info!("test : {:?}", name);
        }
    });
}

pub fn get_other_package_map() -> Result<Map<String, Value>, String> {
    let map: Map<String,Value> = Map::new();

    let contents = read_to_string("checker.json");
    if contents.is_ok(){
        let value = serde_json::from_str::<Value>(contents.ok().unwrap().as_str()).ok().unwrap();

        let object = value.as_object().unwrap();
        let path = object.get("path").unwrap().as_str().unwrap();

        let is_folder = Path::new(path).is_dir();
        if is_folder {
            let package_json_path = &(path.to_owned() + "\\package.json");

            return get_map_from_json(package_json_path)
        }
    } else{
        return Err(String::from("Test"));
    }

    Ok(map)
}

pub fn add_exceptions(package: String){
    let contents = read_to_string("checker.json").ok().unwrap();
    let value = serde_json::from_str::<Value>(contents.as_str()).ok().unwrap();

    let object = value.as_object().unwrap();
    let exception = object.get("exception").unwrap().as_array().unwrap();
    let update = object.get("update").unwrap().as_bool().unwrap();
    let path = object.get("path").unwrap().as_str().unwrap();

    let mut new_exception: Vec<String> = Vec::new();

    exception.iter().for_each(|value|{
        new_exception.push(String::from(value.as_str().unwrap()));
    });

    new_exception.push(package);

    let import_option = ImportOptionBuilder::new().update(update).path(String::from(path))
                                                                .exception(new_exception).build();
    let value = json!(import_option);
    let value = serde_json::to_string_pretty(&value).unwrap();

    let file = OpenOptions::new().write(true).truncate(true).open("checker.json").ok();
    file.unwrap().write_all(value.as_ref()).ok().unwrap();
}