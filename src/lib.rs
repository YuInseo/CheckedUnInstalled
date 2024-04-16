use std::fs::read_to_string;
use std::path::Path;
use serde_json::{Map, Value};

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