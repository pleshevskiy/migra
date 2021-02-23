use super::connection::{DatabaseConnection, DatabaseConnectionManager};
use crate::config::Config;
use crate::StdResult;
use std::convert::TryFrom;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Migration {
    upgrade_sql_file_path: PathBuf,
    downgrade_sql_file_path: PathBuf,
    name: String,
}

impl Migration {
    pub(crate) fn new(directory: &Path) -> Option<Migration> {
        if directory.is_dir() {
            let name = directory
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default();
            let upgrade_sql_file_path = directory.join("up.sql");
            let downgrade_sql_file_path = directory.join("down.sql");

            if upgrade_sql_file_path.exists() && downgrade_sql_file_path.exists() {
                return Some(Migration {
                    upgrade_sql_file_path,
                    downgrade_sql_file_path,
                    name: String::from(name),
                });
            }
        }

        None
    }
}

impl Migration {
    pub fn name(&self) -> &String {
        &self.name
    }

    fn upgrade_sql_content(&self) -> StdResult<String> {
        let content = fs::read_to_string(&self.upgrade_sql_file_path)?;
        Ok(content)
    }

    fn downgrade_sql_content(&self) -> StdResult<String> {
        let content = fs::read_to_string(&self.downgrade_sql_file_path)?;
        Ok(content)
    }
}

pub struct MigrationManager {
    pub(crate) conn: Box<dyn DatabaseConnection>,
}

impl MigrationManager {
    pub fn new(conn: Box<dyn DatabaseConnection>) -> Self {
        MigrationManager { conn }
    }
}

impl TryFrom<&Config> for MigrationManager {
    type Error = Box<dyn std::error::Error>;

    fn try_from(config: &Config) -> Result<Self, Self::Error> {
        let connection_manager = DatabaseConnectionManager::new(&config.database);
        let conn = connection_manager.connect()?;
        Ok(Self { conn })
    }
}

pub fn is_migrations_table_not_found<D: std::fmt::Display>(error: D) -> bool {
    error
        .to_string()
        .contains(r#"relation "migrations" does not exist"#)
}

pub trait DatabaseMigrationManager {
    fn apply_sql(&mut self, sql_content: &str) -> StdResult<()>;

    fn create_migrations_table(&mut self) -> StdResult<()>;

    fn insert_migration_info(&mut self, name: &str) -> StdResult<u64>;

    fn delete_migration_info(&mut self, name: &str) -> StdResult<u64>;

    fn applied_migration_names(&mut self) -> StdResult<Vec<String>>;

    fn upgrade(&mut self, migration: &Migration) -> StdResult<()> {
        let content = migration.upgrade_sql_content()?;

        self.create_migrations_table()?;
        self.apply_sql(&content)?;
        self.insert_migration_info(migration.name())?;

        Ok(())
    }

    fn downgrade(&mut self, migration: &Migration) -> StdResult<()> {
        let content = migration.downgrade_sql_content()?;

        self.apply_sql(&content)?;
        self.delete_migration_info(migration.name())?;

        Ok(())
    }
}

impl DatabaseMigrationManager for MigrationManager {
    fn apply_sql(&mut self, sql_content: &str) -> StdResult<()> {
        self.conn.batch_execute(sql_content)
    }

    fn create_migrations_table(&mut self) -> StdResult<()> {
        self.conn.batch_execute(
            r#"CREATE TABLE IF NOT EXISTS migrations (
                id      serial      PRIMARY KEY,
                name    text        NOT NULL UNIQUE
            )"#,
        )
    }

    fn insert_migration_info(&mut self, name: &str) -> StdResult<u64> {
        self.conn
            .execute("INSERT INTO migrations (name) VALUES ($1)", &[&name])
    }

    fn delete_migration_info(&mut self, name: &str) -> StdResult<u64> {
        self.conn
            .execute("DELETE FROM migrations WHERE name = $1", &[&name])
    }

    fn applied_migration_names(&mut self) -> StdResult<Vec<String>> {
        let res = self
            .conn
            .query("SELECT name FROM migrations ORDER BY id DESC", &[])
            .or_else(|e| {
                if is_migrations_table_not_found(&e) {
                    Ok(Vec::new())
                } else {
                    Err(e)
                }
            })?;

        let applied_migration_names: Vec<String> = res
            .into_iter()
            .filter_map(|row| row.first().cloned())
            .collect();

        Ok(applied_migration_names)
    }
}

pub fn filter_pending_migrations(
    migrations: Vec<Migration>,
    applied_migration_names: &[String],
) -> Vec<Migration> {
    migrations
        .into_iter()
        .filter(|m| !applied_migration_names.contains(m.name()))
        .collect()
}