use crate::config::Config;
use crate::StdResult;
use postgres::{Client, Error, NoTls};
use std::convert::TryFrom;

pub struct PostgresConnection {
    client: Client,
}

impl TryFrom<&Config> for PostgresConnection {
    type Error = Box<dyn std::error::Error>;

    fn try_from(config: &Config) -> Result<Self, Self::Error> {
        PostgresConnection::open(&config.database_connection_string()?)
    }
}

impl PostgresConnection {
    pub fn open(connection_string: &str) -> StdResult<PostgresConnection> {
        let client = Client::connect(connection_string, NoTls)?;
        Ok(PostgresConnection { client })
    }
}

pub fn is_migrations_table_not_found(e: &Error) -> bool {
    e.to_string()
        .contains(r#"relation "migrations" does not exist"#)
}

impl PostgresConnection {
    pub fn apply_sql(&mut self, sql_content: &str) -> Result<(), Error> {
        self.client.batch_execute(sql_content)
    }

    pub fn applied_migration_names(&mut self) -> Result<Vec<String>, Error> {
        let res = self
            .client
            .query("SELECT name FROM migrations ORDER BY id DESC", &[])
            .or_else(|e| {
                if is_migrations_table_not_found(&e) {
                    Ok(Vec::new())
                } else {
                    Err(e)
                }
            })?;

        Ok(res.into_iter().map(|row| row.get(0)).collect())
    }

    pub fn create_migrations_table(&mut self) -> Result<(), Error> {
        self.apply_sql(
            r#"CREATE TABLE IF NOT EXISTS migrations (
                id      serial      PRIMARY KEY,
                name    text        NOT NULL UNIQUE
            )"#,
        )
    }

    pub fn insert_migration_info(&mut self, name: &str) -> Result<u64, Error> {
        self.client
            .execute("INSERT INTO migrations (name) VALUES ($1)", &[&name])
    }

    pub fn delete_migration_info(&mut self, name: &str) -> Result<u64, Error> {
        self.client
            .execute("DELETE FROM migrations WHERE name = $1", &[&name])
    }
}
