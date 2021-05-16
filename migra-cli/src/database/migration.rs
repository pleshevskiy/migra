use super::connection::AnyConnection;
use crate::Config;
use crate::StdResult;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
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

#[derive(Debug)]
pub struct MigrationManager {
    migrations_table_name: String,
}

impl MigrationManager {
    fn new(migrations_table_name: &str) -> Self {
        MigrationManager {
            migrations_table_name: migrations_table_name.to_owned(),
        }
    }
}

impl From<&Config> for MigrationManager {
    fn from(config: &Config) -> Self {
        MigrationManager::new(&config.migrations.table_name())
    }
}

pub fn is_migrations_table_not_found<D: std::fmt::Display>(error: D) -> bool {
    let error_message = error.to_string();

    fn is_postgres_error(error_message: &str) -> bool {
        error_message.contains("relation") && error_message.ends_with("does not exist")
    }

    fn is_mysql_error(error_message: &str) -> bool {
        error_message.contains("ERROR 1146 (42S02)")
    }

    is_postgres_error(&error_message) || is_mysql_error(&error_message)
}

pub trait ManageMigration {
    fn apply_sql(&self, conn: &mut AnyConnection, sql_content: &str) -> StdResult<()>;

    fn create_migrations_table(&self, conn: &mut AnyConnection) -> StdResult<()>;

    fn insert_migration_info(&self, conn: &mut AnyConnection, name: &str) -> StdResult<u64>;

    fn delete_migration_info(&self, conn: &mut AnyConnection, name: &str) -> StdResult<u64>;

    fn applied_migration_names(&self, conn: &mut AnyConnection) -> StdResult<Vec<String>>;

    fn upgrade(&self, conn: &mut AnyConnection, migration: &Migration) -> StdResult<()> {
        let content = migration.upgrade_sql_content()?;

        self.create_migrations_table(conn)?;
        self.apply_sql(conn, &content)?;
        self.insert_migration_info(conn, migration.name())?;

        Ok(())
    }

    fn downgrade(&self, conn: &mut AnyConnection, migration: &Migration) -> StdResult<()> {
        let content = migration.downgrade_sql_content()?;

        self.apply_sql(conn, &content)?;
        self.delete_migration_info(conn, migration.name())?;

        Ok(())
    }
}

impl ManageMigration for MigrationManager {
    fn apply_sql(&self, conn: &mut AnyConnection, sql_content: &str) -> StdResult<()> {
        conn.batch_execute(sql_content)
    }

    fn create_migrations_table(&self, conn: &mut AnyConnection) -> StdResult<()> {
        let stmt = conn.create_migration_table_stmt(&self.migrations_table_name);
        conn.batch_execute(&stmt)
    }

    fn insert_migration_info(&self, conn: &mut AnyConnection, name: &str) -> StdResult<u64> {
        conn.execute(
            &format!(
                "INSERT INTO {} (name) VALUES ($1)",
                &self.migrations_table_name
            ),
            &[&name],
        )
    }

    fn delete_migration_info(&self, conn: &mut AnyConnection, name: &str) -> StdResult<u64> {
        conn.execute(
            &format!(
                "DELETE FROM {} WHERE name = $1",
                &self.migrations_table_name
            ),
            &[&name],
        )
    }

    fn applied_migration_names(&self, conn: &mut AnyConnection) -> StdResult<Vec<String>> {
        let res = conn
            .query(
                &format!(
                    "SELECT name FROM {} ORDER BY id DESC",
                    &self.migrations_table_name
                ),
                &[],
            )
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
