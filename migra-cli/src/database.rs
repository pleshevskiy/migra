use postgres::{Client, Error, NoTls};

pub fn connect(connection_string: &str) -> Result<Client, Error> {
    Client::connect(connection_string, NoTls)
}

pub fn apply_sql(client: &mut Client, sql_content: &str) -> Result<(), Error> {
    client.batch_execute(sql_content)
}

pub fn is_migrations_table_not_found(e: &Error) -> bool {
    e.to_string()
        .contains(r#"relation "migrations" does not exist"#)
}

pub fn applied_migrations(client: &mut Client) -> Result<Vec<String>, Error> {
    let res = client
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

pub fn create_migrations_table(client: &mut Client) -> Result<(), Error> {
    apply_sql(
        client,
        r#"CREATE TABLE IF NOT EXISTS migrations (
            id      serial      PRIMARY KEY,
            name    text        NOT NULL UNIQUE
        )"#,
    )
}

pub fn insert_migration_info(client: &mut Client, name: &str) -> Result<u64, Error> {
    client.execute("INSERT INTO migrations (name) VALUES ($1)", &[&name])
}

pub fn delete_migration_info(client: &mut Client, name: &str) -> Result<u64, Error> {
    client.execute("DELETE FROM migrations WHERE name = $1", &[&name])
}
