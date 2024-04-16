use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageOption {
    pub update: bool,
    pub path: String,
}

impl PackageOption {
    pub fn new() -> Self{
        PackageOption {
            update: false,
            path: String::from("")
        }
    }
}