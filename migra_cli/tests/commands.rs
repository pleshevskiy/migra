pub use assert_cmd::prelude::*;
pub use cfg_if::cfg_if;
use client_mysql::prelude::*;
pub use predicates::str::contains;
pub use std::process::Command;

pub type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

pub const ROOT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/");

pub fn path_to_file<D: std::fmt::Display>(file_name: D) -> String {
    format!("{}{}", ROOT_PATH, file_name)
}

pub fn database_manifest_path<D: std::fmt::Display>(database_name: D) -> String {
    path_to_file(format!("Migra_{}.toml", database_name))
}

pub const DATABASE_URL_DEFAULT_ENV_NAME: &str = "DATABASE_URL";
pub const POSTGRES_URL: &str = "postgres://postgres:postgres@localhost:6000/migra_tests";
pub const MYSQL_URL: &str = "mysql://mysql:mysql@localhost:6001/migra_tests";
pub const SQLITE_URL: &str = "local.db";

pub fn remove_sqlite_db() -> TestResult {
    std::fs::remove_file(SQLITE_URL).or(Ok(()))
}

pub struct Env {
    key: &'static str,
}

impl Env {
    pub fn new(key: &'static str, value: &'static str) -> Self {
        std::env::set_var(key, value);
        Env { key }
    }
}

impl Drop for Env {
    fn drop(&mut self) {
        std::env::remove_var(self.key);
    }
}

mod init {
    use super::*;
    use std::fs;

    #[test]
    fn init_manifest_with_default_config() -> TestResult {
        let manifest_path = "Migra.toml";

        fs::remove_file(&manifest_path).ok();

        Command::cargo_bin("migra")?
            .arg("init")
            .assert()
            .success()
            .stdout(contains(format!("Created {}", &manifest_path)));

        let content = fs::read_to_string(&manifest_path)?;

        assert_eq!(
            content,
            r#"root = "database"

[database]
connection = "$DATABASE_URL"

[migrations]
directory = "migrations"
table_name = "migrations"
"#
        );

        fs::remove_file(&manifest_path)?;

        Ok(())
    }

    #[test]
    fn init_manifest_in_custom_path() -> TestResult {
        let manifest_path = path_to_file("Migra.toml");

        fs::remove_file(&manifest_path).ok();

        Command::cargo_bin("migra")?
            .arg("-c")
            .arg(&manifest_path)
            .arg("init")
            .assert()
            .success()
            .stdout(contains(format!("Created {}", manifest_path.as_str())));

        let content = fs::read_to_string(&manifest_path)?;

        assert_eq!(
            content,
            r#"root = "database"

[database]
connection = "$DATABASE_URL"

[migrations]
directory = "migrations"
table_name = "migrations"
"#
        );

        fs::remove_file(&manifest_path)?;

        Ok(())
    }
}

mod list {
    use super::*;

    #[test]
    fn empty_migration_list() -> TestResult {
        Command::cargo_bin("migra")?
            .arg("ls")
            .assert()
            .success()
            .stderr(contains(
                r#"WARNING: Missed "DATABASE_URL" environment variable
WARNING: No connection to database"#,
            ))
            .stdout(contains(
                r#"
Pending migrations:
—"#,
            ));
        Ok(())
    }

    #[test]
    fn empty_migration_list_with_db() -> TestResult {
        fn inner(connection_string: &'static str) -> TestResult {
            let env = Env::new(DATABASE_URL_DEFAULT_ENV_NAME, connection_string);

            Command::cargo_bin("migra")?
                .arg("ls")
                .assert()
                .success()
                .stdout(contains(
                    r#"Applied migrations:
—

Pending migrations:
—"#,
                ));

            drop(env);

            Ok(())
        }

        #[cfg(feature = "postgres")]
        inner(POSTGRES_URL)?;

        #[cfg(feature = "mysql")]
        inner(MYSQL_URL)?;

        #[cfg(feature = "sqlite")]
        remove_sqlite_db().and_then(|_| inner(SQLITE_URL))?;

        Ok(())
    }

    #[test]
    #[cfg(feature = "postgres")]
    fn empty_migration_list_with_url_in_manifest() -> TestResult {
        Command::cargo_bin("migra")?
            .arg("-c")
            .arg(path_to_file("Migra_url_empty.toml"))
            .arg("ls")
            .assert()
            .success()
            .stdout(contains(
                r#"Applied migrations:
—

Pending migrations:
—"#,
            ));

        Ok(())
    }

    #[test]
    #[cfg(feature = "postgres")]
    fn empty_migration_list_with_env_in_manifest() -> TestResult {
        let env = Env::new("DB_URL", POSTGRES_URL);

        Command::cargo_bin("migra")?
            .arg("-c")
            .arg(path_to_file("Migra_env_empty.toml"))
            .arg("ls")
            .assert()
            .success()
            .stdout(contains(
                r#"Applied migrations:
—

Pending migrations:
—"#,
            ));

        drop(env);

        Ok(())
    }

    #[test]
    fn empty_applied_migrations() -> TestResult {
        fn inner(database_name: &'static str) -> TestResult {
            Command::cargo_bin("migra")?
                .arg("-c")
                .arg(database_manifest_path(database_name))
                .arg("ls")
                .assert()
                .success()
                .stdout(contains(
                    r#"Applied migrations:
—

Pending migrations:
210218232851_create_articles
210218233414_create_persons
"#,
                ));

            Ok(())
        }

        #[cfg(feature = "postgres")]
        inner("postgres")?;

        #[cfg(feature = "mysql")]
        inner("mysql")?;

        #[cfg(feature = "sqlite")]
        remove_sqlite_db().and_then(|_| inner("sqlite"))?;

        Ok(())
    }

    #[test]
    fn applied_all_migrations() -> TestResult {
        fn inner(database_name: &'static str) -> TestResult {
            let manifest_path = database_manifest_path(database_name);

            Command::cargo_bin("migra")?
                .arg("-c")
                .arg(&manifest_path)
                .arg("up")
                .assert()
                .success();

            Command::cargo_bin("migra")?
                .arg("-c")
                .arg(&manifest_path)
                .arg("ls")
                .assert()
                .success()
                .stdout(contains(
                    r#"Applied migrations:
210218232851_create_articles
210218233414_create_persons

Pending migrations:
—
"#,
                ));

            Command::cargo_bin("migra")?
                .arg("-c")
                .arg(&manifest_path)
                .arg("down")
                .arg("--all")
                .assert()
                .success();

            Ok(())
        }

        #[cfg(feature = "postgres")]
        inner("postgres")?;

        #[cfg(feature = "mysql")]
        inner("mysql")?;

        #[cfg(feature = "sqlite")]
        remove_sqlite_db().and_then(|_| inner("sqlite"))?;

        Ok(())
    }

    #[test]
    fn applied_one_migrations() -> TestResult {
        fn inner(database_name: &'static str) -> TestResult {
            let manifest_path = database_manifest_path(database_name);

            Command::cargo_bin("migra")?
                .arg("-c")
                .arg(&manifest_path)
                .arg("up")
                .arg("-n")
                .arg("1")
                .assert()
                .success();

            Command::cargo_bin("migra")?
                .arg("-c")
                .arg(&manifest_path)
                .arg("ls")
                .assert()
                .success()
                .stdout(contains(
                    r#"Applied migrations:
210218232851_create_articles

Pending migrations:
210218233414_create_persons
"#,
                ));

            Command::cargo_bin("migra")?
                .arg("-c")
                .arg(&manifest_path)
                .arg("down")
                .assert()
                .success();

            Ok(())
        }

        #[cfg(feature = "postgres")]
        inner("postgres")?;

        #[cfg(feature = "mysql")]
        inner("mysql")?;

        #[cfg(feature = "sqlite")]
        remove_sqlite_db().and_then(|_| inner("sqlite"))?;

        Ok(())
    }
}

mod make {
    use super::*;
    use std::fs;

    #[test]
    fn make_migration_directory() -> TestResult {
        fn inner(database_name: &'static str) -> TestResult {
            Command::cargo_bin("migra")?
                .arg("-c")
                .arg(database_manifest_path(database_name))
                .arg("make")
                .arg("test")
                .assert()
                .success()
                .stdout(contains("Structure for migration has been created in"));

            let entries = fs::read_dir(path_to_file(format!("{}/migrations", database_name)))?
                .map(|entry| entry.map(|e| e.path()))
                .collect::<Result<Vec<_>, std::io::Error>>()?;

            let dir_paths = entries
                .iter()
                .filter_map(|path| {
                    path.to_str().and_then(|path| {
                        if path.ends_with("_test") {
                            Some(path)
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<_>>();

            for dir_path in dir_paths.iter() {
                let upgrade_content = fs::read_to_string(format!("{}/up.sql", dir_path))?;
                let downgrade_content = fs::read_to_string(format!("{}/down.sql", dir_path))?;

                assert_eq!(upgrade_content, "-- Your SQL goes here\n\n");

                assert_eq!(
                    downgrade_content,
                    "-- This file should undo anything in `up.sql`\n\n"
                );

                fs::remove_dir_all(dir_path)?;
            }

            Ok(())
        }

        #[cfg(feature = "postgres")]
        inner("postgres")?;

        #[cfg(feature = "mysql")]
        inner("mysql")?;

        #[cfg(feature = "sqlite")]
        remove_sqlite_db().and_then(|_| inner("sqlite"))?;

        Ok(())
    }
}

mod upgrade {
    use super::*;

    #[test]
    fn applied_all_migrations() -> TestResult {
        fn inner<ValidateFn>(database_name: &'static str, validate: ValidateFn) -> TestResult
        where
            ValidateFn: Fn() -> TestResult,
        {
            let manifest_path = database_manifest_path(database_name);

            Command::cargo_bin("migra")?
                .arg("-c")
                .arg(&manifest_path)
                .arg("up")
                .assert()
                .success();

            validate()?;

            Command::cargo_bin("migra")?
                .arg("-c")
                .arg(&manifest_path)
                .arg("down")
                .arg("--all")
                .assert()
                .success();

            Ok(())
        }

        #[cfg(feature = "postgres")]
        inner("postgres", || {
            let mut conn = client_postgres::Client::connect(POSTGRES_URL, client_postgres::NoTls)?;
            let res = conn.query("SELECT p.id, a.id FROM persons AS p, articles AS a", &[])?;

            assert_eq!(
                res.into_iter()
                    .map(|row| (row.get(0), row.get(1)))
                    .collect::<Vec<(i32, i32)>>(),
                Vec::new()
            );

            Ok(())
        })?;

        #[cfg(feature = "mysql")]
        inner("mysql", || {
            let pool = client_mysql::Pool::new(MYSQL_URL)?;
            let mut conn = pool.get_conn()?;

            let res = conn.query_drop("SELECT p.id, a.id FROM persons AS p, articles AS a")?;

            assert_eq!(res, ());

            Ok(())
        })?;

        #[cfg(feature = "sqlite")]
        remove_sqlite_db().and_then(|_| {
            inner("sqlite", || {
                let conn = client_rusqlite::Connection::open(SQLITE_URL)?;
                let res =
                    conn.execute_batch("SELECT p.id, a.id FROM persons AS p, articles AS a")?;
                assert_eq!(res, ());

                Ok(())
            })
        })?;

        Ok(())
    }

    #[test]
    fn cannot_applied_invalid_migrations_in_single_transaction() -> TestResult {
        fn inner<ValidateFn>(database_name: &'static str, validate: ValidateFn) -> TestResult
        where
            ValidateFn: Fn() -> TestResult,
        {
            let manifest_path = database_manifest_path(database_name);

            Command::cargo_bin("migra")?
                .arg("-c")
                .arg(&manifest_path)
                .arg("up")
                .arg("--single-transaction")
                .assert()
                .failure();

            validate()?;

            Ok(())
        }

        #[cfg(feature = "postgres")]
        inner("postgres_invalid", || {
            let mut conn = client_postgres::Client::connect(POSTGRES_URL, client_postgres::NoTls)?;
            let articles_res = conn.query("SELECT a.id FROM articles AS a", &[]);
            let persons_res = conn.query("SELECT p.id FROM persons AS p", &[]);

            assert!(articles_res.is_err());
            assert!(persons_res.is_err());

            Ok(())
        })?;

        #[cfg(feature = "sqlite")]
        remove_sqlite_db().and_then(|_| {
            inner("sqlite_invalid", || {
                let conn = client_rusqlite::Connection::open(SQLITE_URL)?;
                let articles_res = conn.execute_batch("SELECT a.id FROM articles AS a");
                let persons_res = conn.execute_batch("SELECT p.id FROM persons AS p");

                assert!(articles_res.is_err());
                assert!(persons_res.is_err());

                Ok(())
            })
        })?;

        // mysql doesn't support DDL in transaction 🤷

        Ok(())
    }
}

mod apply {
    use super::*;

    #[test]
    fn apply_files() -> TestResult {
        fn inner<ValidateFn>(
            database_name: &'static str,
            file_paths: Vec<&'static str>,
            validate: ValidateFn,
        ) -> TestResult
        where
            ValidateFn: Fn() -> TestResult,
        {
            let manifest_path = database_manifest_path(database_name);

            Command::cargo_bin("migra")?
                .arg("-c")
                .arg(&manifest_path)
                .arg("apply")
                .args(file_paths)
                .assert()
                .success();

            validate()?;

            Ok(())
        }

        cfg_if! {
            if #[cfg(feature = "postgres")] {
                inner(
                    "postgres",
                    vec![
                        "migrations/210218232851_create_articles/up",
                        "migrations/210218233414_create_persons/up",
                    ],
                    || {
                        let mut conn = client_postgres::Client::connect(POSTGRES_URL, client_postgres::NoTls)?;
                        let res = conn.query("SELECT p.id, a.id FROM persons AS p, articles AS a", &[])?;

                        assert_eq!(
                            res.into_iter()
                                .map(|row| (row.get(0), row.get(1)))
                                .collect::<Vec<(i32, i32)>>(),
                            Vec::new()
                        );

                        Ok(())
                    },
                )?;

                inner(
                    "postgres",
                    vec![
                        "migrations/210218233414_create_persons/down",
                        "migrations/210218232851_create_articles/down",
                    ],
                    || {
                        let mut conn = client_postgres::Client::connect(POSTGRES_URL, client_postgres::NoTls)?;
                        let res = conn.query("SELECT p.id, a.id FROM persons AS p, articles AS a", &[]);

                        assert!(res.is_err());

                        Ok(())
                    },
                )?;
            }
        }

        cfg_if! {
            if #[cfg(feature = "mysql")] {
                inner(
                    "mysql",
                    vec![
                        "migrations/210218232851_create_articles/up",
                        "migrations/210218233414_create_persons/up",
                    ],
                    || {
                        let pool = client_mysql::Pool::new(MYSQL_URL)?;
                        let mut conn = pool.get_conn()?;

                        let res = conn.query_drop("SELECT p.id, a.id FROM persons AS p, articles AS a")?;

                        assert_eq!(res, ());

                        Ok(())
                    },
                )?;

                inner(
                    "mysql",
                    vec![
                        "migrations/210218233414_create_persons/down",
                        "migrations/210218232851_create_articles/down",
                    ],
                    || {
                        let pool = client_mysql::Pool::new(MYSQL_URL)?;
                        let mut conn = pool.get_conn()?;

                        let res = conn.query_drop("SELECT p.id, a.id FROM persons AS p, articles AS a");

                        assert!(res.is_err());

                        Ok(())
                    }
                )?;
            }
        }

        #[cfg(feature = "sqlite")]
        remove_sqlite_db().and_then(|_| {
            inner(
                "sqlite",
                vec![
                    "migrations/210218232851_create_articles/up",
                    "migrations/210218233414_create_persons/up",
                ],
                || {
                    let conn = client_rusqlite::Connection::open(SQLITE_URL)?;
                    let res =
                        conn.execute_batch("SELECT p.id, a.id FROM persons AS p, articles AS a")?;
                    assert_eq!(res, ());

                    Ok(())
                },
            )?;

            inner(
                "sqlite",
                vec![
                    "migrations/210218233414_create_persons/down",
                    "migrations/210218232851_create_articles/down",
                ],
                || {
                    let conn = client_rusqlite::Connection::open(SQLITE_URL)?;
                    let res =
                        conn.execute_batch("SELECT p.id, a.id FROM persons AS p, articles AS a");
                    assert!(res.is_err());

                    Ok(())
                },
            )
        })?;

        Ok(())
    }
}
