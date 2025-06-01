use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub mapper: MapperConfig,
}

#[derive(Debug, Deserialize)]
pub struct MapperConfig {
    pub endpoint: String,
}

pub fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let raw = fs::read_to_string("scanner_config.toml")?;
    let config: Config = toml::from_str(&raw)?;
    Ok(config)
}

