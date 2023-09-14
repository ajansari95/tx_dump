use std::fs;
use std::io::{BufReader, Read};
use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize,Clone)]
pub struct Config {
    pub(crate) url: String,
}

impl Config{
    pub fn url(&self) -> &String{
        &self.url
    }

    pub fn from_file(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config:Config =toml::from_str(&contents)?;
        println!("Config: {:?}", config);
        Ok(config)
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            url: "https://localhost:1317".to_string(),
        }
    }

}