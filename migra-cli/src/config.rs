use crate::database;
use crate::path::PathBuilder;
use postgres::Client;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::{env, fs, io};

const MIGRA_TOML_FILENAME: &str = "Migra.toml";
const DEFAULT_DATABASE_CONNECTION_ENV: &str = "$DATABASE_URL";

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Config {
    #[serde(skip)]
    root: PathBuf,

    directory: PathBuf,

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
            root: PathBuf::new(),
            directory: PathBuf::from("database"),
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
                config.root = config_path
                    .parent()
                    .unwrap_or_else(|| Path::new(""))
                    .to_path_buf();

                Ok(config)
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

impl Config {
    pub fn directory_path(&self) -> PathBuf {
        PathBuilder::from(&self.root)
            .append(&self.directory)
            .build()
    }

    pub fn database_connection(&self) -> String {
        let connection = self
            .database
            .connection
            .clone()
            .unwrap_or_else(|| String::from(DEFAULT_DATABASE_CONNECTION_ENV));
        if let Some(connection_env) = connection.strip_prefix("$") {
            env::var(connection_env).unwrap_or_else(|_| {
                panic!(
                    r#"You need to provide "{}" environment variable"#,
                    connection_env
                )
            })
        } else {
            connection
        }
    }

    pub fn migration_dir_path(&self) -> PathBuf {
        PathBuilder::from(&self.directory_path())
            .append("migrations")
            .build()
    }

    pub fn migrations(&self) -> io::Result<Vec<Migration>> {
        let mut entries = self
            .migration_dir_path()
            .read_dir()?
            .map(|res| res.map(|e| e.path()))
            .collect::<Result<Vec<_>, io::Error>>()?;

        entries.sort();

        let migrations = entries
            .iter()
            .filter_map(Migration::new)
            .collect::<Vec<_>>();

        Ok(migrations)
    }
}

#[derive(Debug)]
pub struct Migration {
    upgrade_sql: PathBuf,
    downgrade_sql: PathBuf,
    name: String,
}

impl Migration {
    fn new(directory: &PathBuf) -> Option<Migration> {
        if directory.is_dir() {
            let name = directory
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default();
            let upgrade_sql = PathBuilder::from(directory).append("up.sql").build();
            let downgrade_sql = PathBuilder::from(directory).append("down.sql").build();

            if upgrade_sql.exists() && downgrade_sql.exists() {
                return Some(Migration {
                    upgrade_sql,
                    downgrade_sql,
                    name: String::from(name),
                });
            }
        }

        None
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn upgrade(&self, client: &mut Client) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let content = fs::read_to_string(&self.upgrade_sql)?;

        database::create_migration_table(client)?;

        database::apply_sql(client, &content)?;

        database::insert_migration_info(client, self.name())?;

        Ok(())
    }

    pub fn downgrade(
        &self,
        client: &mut Client,
    ) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let content = fs::read_to_string(&self.downgrade_sql)?;

        database::apply_sql(client, &content)?;

        database::delete_migration_info(client, self.name())?;

        Ok(())
    }
}
