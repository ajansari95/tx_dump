use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub(crate) url: String,
}

impl Config{
    pub fn url(&self) -> &String{
        &self.url
    }

    pub fn from_file(path: &str) -> Result<Config, std::io::Error> {
        let file = std::fs::File::open(path)?;
        let reader = std::io::BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
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