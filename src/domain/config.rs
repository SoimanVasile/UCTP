use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config{
    pub start_temp: f64,
    pub cooling_rate: f64,
    pub max_iterations: u32,
    pub file_name: String,
}

impl Config{
    pub fn load() -> Result<Self, Box<dyn std::error::Error>>{
        let content = fs::read_to_string("config.toml")?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
