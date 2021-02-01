use migra_core::path::PathBuilder;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const MIGRA_TOML_FILENAME: &str = "Migra.toml";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    #[serde(skip)]
    pub root: PathBuf,
    pub directory: PathBuf,
    pub database: DatabaseConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct DatabaseConfig {
    pub connection: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            root: PathBuf::new(),
            directory: PathBuf::from("database"),
            database: DatabaseConfig {
                connection: String::new(),
            },
        }
    }
}

impl Config {
    pub fn read() -> io::Result<Config> {
        let current_dir = std::env::current_dir()?;

        let mut read_dir = Some(current_dir.as_path());

        loop {
            if let Some(dir) = read_dir {
                let migra_file_path = PathBuilder::from(dir).append(MIGRA_TOML_FILENAME).build();
                if !migra_file_path.exists() {
                    read_dir = dir.parent();
                    continue;
                }

                let content = fs::read_to_string(migra_file_path)?;
                let mut config: Config = toml::from_str(&content).expect("Cannot parse Migra.toml");

                config.root = PathBuf::from(dir);

                return Ok(config);
            } else {
                return Err(io::Error::from(io::ErrorKind::NotFound));
            }
        }
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
