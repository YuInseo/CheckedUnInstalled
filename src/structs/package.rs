use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Packages{
    pub name: String,
    pub version: String,
    pub excepted: bool
}

impl Packages{
    pub fn new(name: String, version: String) -> Self{
        Packages{
            name,
            version,
            excepted: false,
        }
    }
}