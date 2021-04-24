use super::connection::AnyConnection;
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
pub struct MigrationManager;

impl MigrationManager {
    pub fn new() -> Self {
        MigrationManager
    }
}

pub fn is_migrations_table_not_found<D: std::fmt::Display>(error: D) -> bool {
    let error_message = error.to_string();

    // Postgres error
    error_message.contains(r#"relation "migrations" does not exist"#)
        // MySQL error
        || error_message.contains("ERROR 1146 (42S02)")
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
        let stmt = conn.create_migration_table_stmt();
        conn.batch_execute(&stmt)
    }

    fn insert_migration_info(&self, conn: &mut AnyConnection, name: &str) -> StdResult<u64> {
        conn.execute("INSERT INTO migrations (name) VALUES ($1)", &[&name])
    }

    fn delete_migration_info(&self, conn: &mut AnyConnection, name: &str) -> StdResult<u64> {
        conn.execute("DELETE FROM migrations WHERE name = $1", &[&name])
    }

    fn applied_migration_names(&self, conn: &mut AnyConnection) -> StdResult<Vec<String>> {
        let res = conn
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
