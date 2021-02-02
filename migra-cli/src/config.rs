use migra_core::path::PathBuilder;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{fs, io};

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

fn recursive_find_config_file() -> io::Result<PathBuf> {
    let current_dir = std::env::current_dir()?;

    let mut read_dir = Some(current_dir.as_path());

    loop {
        if let Some(dir) = read_dir {
            let migra_file_path = PathBuilder::from(dir).append(MIGRA_TOML_FILENAME).build();
            if !migra_file_path.exists() {
                read_dir = dir.parent();
                continue;
            }

            return Ok(migra_file_path);
        } else {
            return Err(io::Error::from(io::ErrorKind::NotFound));
        }
    }
}

impl Config {
    pub fn read(config_path: Option<PathBuf>) -> io::Result<Config> {
        let config_path = match config_path {
            Some(mut config_path) if config_path.is_dir() => {
                config_path.push(MIGRA_TOML_FILENAME);
                config_path
            }
            Some(config_path) => config_path,
            None => recursive_find_config_file()?,
        };

        let content = fs::read_to_string(&config_path)?;

        let mut config: Config = toml::from_str(&content).expect("Cannot parse Migra.toml");
        config.root = config_path
            .parent()
            .unwrap_or_else(|| Path::new(""))
            .to_path_buf();

        Ok(config)
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

impl Config {
    pub fn directory_path(&self) -> PathBuf {
        PathBuilder::from(&self.root)
            .append(&self.directory)
            .build()
    }

    pub fn migration_dirs(&self) -> io::Result<Vec<PathBuf>> {
        let mut entries = self
            .directory_path()
            .read_dir()?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        entries.sort();

        let migration_dir_entries = entries
            .into_iter()
            .filter(|entry| {
                entry.is_dir()
                    && PathBuilder::from(entry).append("up.sql").build().exists()
                    && PathBuilder::from(entry).append("down.sql").build().exists()
            })
            .collect::<Vec<_>>();

        Ok(migration_dir_entries)
    }
}
