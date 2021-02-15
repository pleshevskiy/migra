use crate::database::{DatabaseConnection, PostgresConnection};
use crate::path::PathBuilder;
use crate::StdResult;
use std::fs;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Migration {
    upgrade_sql: PathBuf,
    downgrade_sql: PathBuf,
    name: String,
}

impl Migration {
    pub(crate) fn new(directory: &PathBuf) -> Option<Migration> {
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
}

impl Migration {
    pub fn name(&self) -> &String {
        &self.name
    }

    fn upgrade_sql_content(&self) -> StdResult<String> {
        let content = fs::read_to_string(&self.upgrade_sql)?;
        Ok(content)
    }

    fn downgrade_sql_content(&self) -> StdResult<String> {
        let content = fs::read_to_string(&self.downgrade_sql)?;
        Ok(content)
    }
}

pub struct MigrationManager<Conn: DatabaseConnection> {
    conn: Conn,
}

impl<Conn: DatabaseConnection> MigrationManager<Conn> {
    pub fn new(conn: Conn) -> Self {
        MigrationManager { conn }
    }
}

pub fn is_migrations_table_not_found<D: std::fmt::Display>(error: D) -> bool {
    error
        .to_string()
        .contains(r#"relation "migrations" does not exist"#)
}

pub trait DatabaseMigrationManager {
    const CREATE_MIGRATIONS_STMT: &'static str = r#"
        CREATE TABLE IF NOT EXISTS migrations (
            id      serial      PRIMARY KEY,
            name    text        NOT NULL UNIQUE
        )
    "#;

    const INSERT_MIGRATION_STMT: &'static str = "INSERT INTO migrations (name) VALUES ($1)";

    const DELETE_MIGRATION_STMT: &'static str = "DELETE FROM migrations WHERE name = $1";

    fn apply_sql(&mut self, sql_content: &str) -> StdResult<()>;

    fn create_migrations_table(&mut self) -> StdResult<()>;

    fn insert_migration_info(&mut self, name: &str) -> StdResult<u64>;

    fn delete_migration_info(&mut self, name: &str) -> StdResult<u64>;

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

impl<Conn> DatabaseMigrationManager for MigrationManager<Conn>
where
    Conn: DatabaseConnection,
{
    fn apply_sql(&mut self, sql_content: &str) -> StdResult<()> {
        self.conn.batch_execute(sql_content)
    }

    fn create_migrations_table(&mut self) -> StdResult<()> {
        self.conn.batch_execute(Self::CREATE_MIGRATIONS_STMT)
    }

    fn insert_migration_info(&mut self, name: &str) -> StdResult<u64> {
        self.conn.execute(Self::INSERT_MIGRATION_STMT, &[&name])
    }

    fn delete_migration_info(&mut self, name: &str) -> StdResult<u64> {
        self.conn.execute(Self::DELETE_MIGRATION_STMT, &[&name])
    }
}

pub trait MigrationNames {
    const APPLIED_MIGRATIONS_STMT: &'static str = "SELECT name FROM migrations ORDER BY id DESC";

    fn applied_migration_names(&mut self) -> StdResult<Vec<String>>;
}

impl MigrationNames for MigrationManager<PostgresConnection> {
    fn applied_migration_names(&mut self) -> StdResult<Vec<String>> {
        let res = self
            .conn
            .query(Self::APPLIED_MIGRATIONS_STMT, &[])
            .or_else(|e| {
                if is_migrations_table_not_found(&e) {
                    Ok(Vec::new())
                } else {
                    Err(e)
                }
            })?;

        Ok(res.into_iter().collect())
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
