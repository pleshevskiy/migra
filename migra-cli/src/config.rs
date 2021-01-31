use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

const MIGRA_TOML_FILENAME: &str = "Migra.toml";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    directory: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            directory: String::from("database"),
        }
    }
}

impl Config {
    pub fn read() -> Config {
        fs::read_to_string(MIGRA_TOML_FILENAME)
            .ok()
            .and_then(|content| toml::from_str(&content).ok())
            .unwrap_or_default()
    }

    pub fn initialize() -> Result<(), Box<dyn std::error::Error>> {
        if Path::new(MIGRA_TOML_FILENAME).exists() {
            println!("{} already exists", MIGRA_TOML_FILENAME);
            return Ok(());
        }

        let config = Config::default();
        let content = toml::to_string(&config)?;
        fs::write(MIGRA_TOML_FILENAME, content)?;

        println!("Created {}", MIGRA_TOML_FILENAME);

        Ok(())
    }
}
