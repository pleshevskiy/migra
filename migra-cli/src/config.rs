use crate::error::{Error, ErrorKind};
use crate::migration::Migration;
use crate::path::PathBuilder;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub(crate) const MIGRA_TOML_FILENAME: &str = "Migra.toml";
pub(crate) const DEFAULT_DATABASE_CONNECTION_ENV: &str = "$DATABASE_URL";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    #[serde(skip)]
    manifest_root: PathBuf,

    root: PathBuf,

    #[serde(default)]
    database: DatabaseConfig,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct DatabaseConfig {
    pub connection: Option<String>,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            manifest_root: PathBuf::new(),
            root: PathBuf::from("database"),
            database: DatabaseConfig {
                connection: Some(String::from(DEFAULT_DATABASE_CONNECTION_ENV)),
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
                Some(config_path)
            }
            Some(config_path) => Some(config_path),
            None => recursive_find_config_file().ok(),
        };

        match config_path {
            None => Ok(Config::default()),
            Some(config_path) => {
                let content = fs::read_to_string(&config_path)?;

                let mut config: Config = toml::from_str(&content).expect("Cannot parse Migra.toml");
                config.manifest_root = config_path
                    .parent()
                    .unwrap_or_else(|| Path::new(""))
                    .to_path_buf();

                Ok(config)
            }
        }
    }
}

impl Config {
    pub fn directory_path(&self) -> PathBuf {
        PathBuilder::from(&self.manifest_root)
            .append(&self.root)
            .build()
    }

    pub fn database_connection_string(&self) -> crate::error::Result<String> {
        let connection = self
            .database
            .connection
            .clone()
            .unwrap_or_else(|| String::from(DEFAULT_DATABASE_CONNECTION_ENV));
        if let Some(connection_env) = connection.strip_prefix("$") {
            env::var(connection_env)
                .map_err(|e| Error::new(ErrorKind::MissedEnvVar(connection_env.to_string()), e))
        } else {
            Ok(connection)
        }
    }

    pub fn migration_dir_path(&self) -> PathBuf {
        PathBuilder::from(&self.directory_path())
            .append("migrations")
            .build()
    }

    pub fn migrations(&self) -> io::Result<Vec<Migration>> {
        let mut entries = match self.migration_dir_path().read_dir() {
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
            entries => entries?
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, io::Error>>()?,
        };

        if entries.is_empty() {
            return Ok(vec![]);
        }

        entries.sort();

        let migrations = entries
            .iter()
            .filter_map(Migration::new)
            .collect::<Vec<_>>();

        Ok(migrations)
    }
}
