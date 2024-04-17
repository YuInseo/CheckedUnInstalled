use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportOption {
    pub update: bool,
    pub path: String,
    pub exception: Vec<String>
}

impl ImportOption {
    pub fn new() -> Self{
        ImportOption {
            update: false,
            path: String::from(""),
            exception: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportOptionBuilder {
    pub update: bool,
    pub path: String,
    pub exception: Vec<String>
}


impl ImportOptionBuilder{
    pub fn new()-> Self{
        ImportOptionBuilder{
            update: false,
            path: "".to_string(),
            exception: vec![],
        }
    }

    pub fn update(mut self, update: bool) -> Self{
        self.update = update;
        self
    }

    pub fn path(mut self, path: String) -> Self{
        self.path = path;
        self
    }

    pub fn exception(mut self, exception: Vec<String>) -> Self{
        self.exception = exception;
        self
    }

    pub fn build(self) -> ImportOption{
        ImportOption{
            update: self.update,
            path: self.path,
            exception: self.exception,
        }
    }
}