# Migra

[![CI](https://github.com/pleshevskiy/migra/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/pleshevskiy/migra/actions/workflows/rust.yml)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
[![Crates.io](https://img.shields.io/crates/v/migra)](https://crates.io/crates/migra)
![Crates.io](https://img.shields.io/crates/l/migra)

Migra is a simple library for managing SQL in your application.

For example, if you have a task list application, you can update the local user database from version to version.

This is main crate for [migra-cli](https://crates.io/crates/migra-cli), which allows you to manege SQL for web
servers in any program language without being bound to SQL frameworks.


### Installation

Add `migra = { version = "1.0" }` as a dependency in `Cargo.toml`.

This crate has not required predefined database clients in features with similar name.
If you want to add them, just install crate with additional features (`postgres`, `mysql`, `sqlite`).

`Cargo.toml` example:

```toml
[package]
name = "my-crate"
version = "0.1.0"
authors = ["Me <user@rust-lang.org>"]

[dependencies]
migra = { version = "1.0", features = ["postgres"] }
```

## Basic usage

**Note:** This example requires to enable `sqlite` feature.

```rust
use migra::clients::{OpenDatabaseConnection, SqliteClient};
use migra::managers::{ManageTransaction, ManageMigrations};

fn main() -> migra::Result<()> {
    let mut client = SqliteClient::new("./tasks.db")?;

    client.create_migrations_table()?;

    let mut migrations = client.get_applied_migrations()?;

    client
        .begin_transaction()
        .and_then(|_| {
            migrations.should_run_upgrade_migration(
                &mut client,
                "20210615_initial_migration",
                r#"CREATE TABLE IF NOT EXISTS tasks (
                    title TEXT NOT NULL
                );"#,
            )?;

            Ok(())
        })
        .and_then(|res| client.commit_transaction().and(Ok(res)))
        .or_else(|err| client.rollback_transaction().and(Err(err)));

    Ok(())
}
```

### Supported databases

| Database | Feature      |
|----------|--------------|
| Postgres | postgres     |
| MySQL    | mysql        |
| Sqlite   | sqlite       |


## License

Licensed under either of these:

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE_APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE_MIT) or
   https://opensource.org/licenses/MIT)
