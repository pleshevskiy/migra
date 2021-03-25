use crate::database::migration::Migration;
use crate::error::{Error, MigraResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{env, fs, io};

pub(crate) const MIGRA_TOML_FILENAME: &str = "Migra.toml";
pub(crate) const DEFAULT_DATABASE_CONNECTION_ENV: &str = "$DATABASE_URL";

fn default_database_connection_env() -> String {
    DEFAULT_DATABASE_CONNECTION_ENV.to_owned()
}

#[cfg(any(not(feature = "postgres"), not(feature = "mysql")))]
macro_rules! please_install_with {
    (feature $database_name:expr) => {
        panic!(
            r#"You cannot use migra for "{database_name}".
You need to reinstall crate with "{database_name}" feature.

cargo install migra-cli --features ${database_name}"#,
            database_name = $database_name
        );
    };
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    #[serde(skip)]
    manifest_root: PathBuf,

    root: PathBuf,

    #[serde(default)]
    pub database: DatabaseConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum SupportedDatabaseClient {
    #[cfg(feature = "postgres")]
    Postgres,
    #[cfg(feature = "mysql")]
    Mysql,
}

impl Default for SupportedDatabaseClient {
    fn default() -> Self {
        cfg_if! {
            if #[cfg(feature = "postgres")] {
                SupportedDatabaseClient::Postgres
            } else if #[cfg(feature = "mysql")] {
                SupportedDatabaseClient::Mysql
            }
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct DatabaseConfig {
    pub client: Option<SupportedDatabaseClient>,

    #[serde(default = "default_database_connection_env")]
    pub connection: String,
}

impl DatabaseConfig {
    pub fn client(&self) -> SupportedDatabaseClient {
        self.client.clone().unwrap_or_else(|| {
            self.connection_string()
                .ok()
                .and_then(|connection_string| {
                    if connection_string.starts_with("postgres://") {
                        cfg_if! {
                            if #[cfg(feature = "postgres")] {
                                Some(SupportedDatabaseClient::Postgres)
                            } else {
                                please_install_with!(feature "postgres")
                            }
                        }
                    } else if connection_string.starts_with("mysql://") {
                        cfg_if! {
                            if #[cfg(feature = "mysql")] {
                                Some(SupportedDatabaseClient::Mysql)
                            } else {
                                please_install_with!(feature "mysql")
                            }
                        }
                    } else {
                        None
                    }
                })
                .unwrap_or_default()
        })
    }

    pub fn connection_string(&self) -> MigraResult<String> {
        if let Some(connection_env) = self.connection.strip_prefix("$") {
            env::var(connection_env).map_err(|_| Error::MissedEnvVar(connection_env.to_string()))
        } else {
            Ok(self.connection.clone())
        }
    }
}

impl Default for Config {
    fn default() -> Config {
        Config {
            manifest_root: PathBuf::default(),
            root: PathBuf::from("database"),
            database: DatabaseConfig {
                connection: default_database_connection_env(),
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
