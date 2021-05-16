use crate::database::migration::Migration;
use crate::error::{Error, MigraResult};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{env, fs, io};

//===========================================================================//
// Internal Config Utils / Macros                                            //
//===========================================================================//

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

//===========================================================================//
// Database config                                                           //
//===========================================================================//

fn default_database_connection_env() -> String {
    String::from("$DATABASE_URL")
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct DatabaseConfig {
    pub client: Option<SupportedDatabaseClient>,

    #[serde(default = "default_database_connection_env")]
    pub connection: String,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        DatabaseConfig {
            connection: default_database_connection_env(),
            client: None,
        }
    }
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

//===========================================================================//
// Migrations config                                                         //
//===========================================================================//

fn default_migrations_directory() -> String {
    String::from("migrations")
}

fn default_migrations_table_name() -> String {
    String::from("migrations")
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct MigrationsConfig {
    #[serde(rename = "directory", default = "default_migrations_directory")]
    directory: String,

    #[serde(default = "default_migrations_table_name")]
    table_name: String,

    date_format: Option<String>,
}

impl Default for MigrationsConfig {
    fn default() -> Self {
        MigrationsConfig {
            directory: default_migrations_directory(),
            table_name: default_migrations_table_name(),
            date_format: None,
        }
    }
}

impl MigrationsConfig {
    pub fn directory(&self) -> String {
        if let Some(directory_env) = self.directory.strip_prefix("$") {
            env::var(directory_env).unwrap_or_else(|_| {
                println!(
                    "WARN: Cannot read {} variable and use {} directory by default",
                    directory_env,
                    default_migrations_directory()
                );
                default_migrations_directory()
            })
        } else {
            self.directory.clone()
        }
    }

    pub fn table_name(&self) -> String {
        if let Some(table_name_env) = self.table_name.strip_prefix("$") {
            env::var(table_name_env).unwrap_or_else(|_| {
                println!(
                    "WARN: Cannot read {} variable and use {} table_name by default",
                    table_name_env,
                    default_migrations_table_name()
                );
                default_migrations_table_name()
            })
        } else {
            self.table_name.clone()
        }
    }

    pub fn date_format(&self) -> String {
        self.date_format
            .clone()
            .unwrap_or_else(|| String::from("%y%m%d%H%M%S"))
    }
}

//===========================================================================//
// Main config                                                               //
//===========================================================================//

pub(crate) const MIGRA_TOML_FILENAME: &str = "Migra.toml";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    #[serde(skip)]
    manifest_root: PathBuf,

    root: PathBuf,

    #[serde(default)]
    pub(crate) database: DatabaseConfig,

    #[serde(default)]
    pub(crate) migrations: MigrationsConfig,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            manifest_root: PathBuf::default(),
            root: PathBuf::from("database"),
            database: DatabaseConfig::default(),
            migrations: MigrationsConfig::default(),
        }
    }
}

impl Config {
    pub fn read(config_path: Option<&PathBuf>) -> MigraResult<Config> {
        let config_path = match config_path {
            Some(config_path) if config_path.is_dir() => {
                Some(config_path.join(MIGRA_TOML_FILENAME))
            }
            Some(config_path) => Some(config_path.clone()),
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

    pub fn directory_path(&self) -> PathBuf {
        self.manifest_root.join(&self.root)
    }

    pub fn migration_dir_path(&self) -> PathBuf {
        self.directory_path().join(self.migrations.directory())
    }

    pub fn migrations(&self) -> MigraResult<Vec<Migration>> {
        let mut entries = match self.migration_dir_path().read_dir() {
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
            entries => entries?
                .map(|res| res.map(|e| e.path()))
                .collect::<Result<Vec<_>, _>>()?,
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
