use crate::database::migration::Migration;
use crate::error::{Error, MigraResult};
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
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) enum SupportedDatabaseClient {
    Postgres,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct DatabaseConfig {
    pub client: Option<SupportedDatabaseClient>,
    pub connection: Option<String>,
}

impl DatabaseConfig {
    pub fn client(&self) -> MigraResult<SupportedDatabaseClient> {
        Ok(SupportedDatabaseClient::Postgres)
    }

    pub fn connection_string(&self) -> MigraResult<String> {
        let connection = self
            .connection
            .clone()
            .unwrap_or_else(|| String::from(DEFAULT_DATABASE_CONNECTION_ENV));
        if let Some(connection_env) = connection.strip_prefix("$") {
            env::var(connection_env).map_err(|_| Error::MissedEnvVar(connection_env.to_string()))
        } else {
            Ok(connection)
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            manifest_root: PathBuf::default(),
            root: PathBuf::from("database"),
            database: DatabaseConfig {
                connection: Some(String::from(DEFAULT_DATABASE_CONNECTION_ENV)),
                ..Default::default()
            },
        }
    }
}

fn search_for_directory_containing_file(path: &Path, file_name: &str) -> MigraResult<PathBuf> {
    let file_path = path.join(file_name);
    if file_path.is_file() {
        Ok(path.to_owned())
    } else {
        path.parent()
            .ok_or(Error::RootNotFound)
            .and_then(|p| search_for_directory_containing_file(p, file_name))
    }
}

fn recursive_find_project_root() -> MigraResult<PathBuf> {
    let current_dir = std::env::current_dir()?;

    search_for_directory_containing_file(&current_dir, MIGRA_TOML_FILENAME)
}

impl Config {
    pub fn read(config_path: Option<PathBuf>) -> MigraResult<Config> {
        let config_path = match config_path {
            Some(config_path) if config_path.is_dir() => {
                Some(config_path.join(MIGRA_TOML_FILENAME))
            }
            Some(config_path) => Some(config_path),
            None => recursive_find_project_root()
                .map(|path| path.join(MIGRA_TOML_FILENAME))
                .ok(),
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
        self.manifest_root.join(&self.root)
    }

    pub fn migration_dir_path(&self) -> PathBuf {
        self.directory_path().join("migrations")
    }

    pub fn migrations(&self) -> MigraResult<Vec<Migration>> {
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
            .filter_map(|path| Migration::new(&path))
            .collect::<Vec<_>>();

        Ok(migrations)
    }
}
