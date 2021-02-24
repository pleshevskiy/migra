pub use assert_cmd::prelude::*;
pub use predicates::str::contains;
pub use std::process::Command;

pub type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

pub const ROOT_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/");

pub fn path_to_file(file_name: &'static str) -> String {
    ROOT_PATH.to_owned() + file_name
}

pub const DATABASE_URL_DEFAULT_ENV_NAME: &str = "DATABASE_URL";
pub const DATABASE_URL_ENV_VALUE: &str = "postgres://postgres:postgres@localhost:6000/migra_tests";

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
"#
        );

        fs::remove_file(&manifest_path)?;

        Ok(())
    }

    #[test]
    fn init_manifest_in_custom_path() -> TestResult {
        let manifest_path = path_to_file("Migra.toml");

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
        let env = Env::new(DATABASE_URL_DEFAULT_ENV_NAME, DATABASE_URL_ENV_VALUE);

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

    #[test]
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
    fn empty_migration_list_with_env_in_manifest() -> TestResult {
        let env = Env::new("DB_URL", DATABASE_URL_ENV_VALUE);

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
        let env = Env::new(DATABASE_URL_DEFAULT_ENV_NAME, DATABASE_URL_ENV_VALUE);

        Command::cargo_bin("migra")?
            .arg("-c")
            .arg(path_to_file("Migra_env.toml"))
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

        drop(env);

        Ok(())
    }

    #[test]
    fn applied_all_migrations() -> TestResult {
        let env = Env::new(DATABASE_URL_DEFAULT_ENV_NAME, DATABASE_URL_ENV_VALUE);

        Command::cargo_bin("migra")?
            .arg("-c")
            .arg(path_to_file("Migra_env.toml"))
            .arg("up")
            .assert()
            .success();

        Command::cargo_bin("migra")?
            .arg("-c")
            .arg(path_to_file("Migra_env.toml"))
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
            .arg(path_to_file("Migra_env.toml"))
            .arg("down")
            .arg("-n")
            .arg("2")
            .assert()
            .success();

        drop(env);

        Ok(())
    }

    #[test]
    fn applied_one_migrations() -> TestResult {
        let env = Env::new(DATABASE_URL_DEFAULT_ENV_NAME, DATABASE_URL_ENV_VALUE);

        Command::cargo_bin("migra")?
            .arg("-c")
            .arg(path_to_file("Migra_env.toml"))
            .arg("up")
            .assert()
            .success();

        Command::cargo_bin("migra")?
            .arg("-c")
            .arg(path_to_file("Migra_env.toml"))
            .arg("down")
            .assert()
            .success();

        Command::cargo_bin("migra")?
            .arg("-c")
            .arg(path_to_file("Migra_env.toml"))
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
            .arg(path_to_file("Migra_env.toml"))
            .arg("down")
            .assert()
            .success();

        drop(env);

        Ok(())
    }
}

mod make {
    use super::*;
    use std::fs;

    #[test]
    fn make_migration_directory() -> TestResult {
        Command::cargo_bin("migra")?
            .arg("-c")
            .arg(path_to_file("Migra_url.toml"))
            .arg("make")
            .arg("test")
            .assert()
            .success()
            .stdout(contains("Structure for migration has been created in"));

        let entries = fs::read_dir(path_to_file("migrations"))?
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
}

mod upgrade {
    use super::*;

    #[test]
    fn applied_all_migrations() -> TestResult {
        let env = Env::new(DATABASE_URL_DEFAULT_ENV_NAME, DATABASE_URL_ENV_VALUE);

        Command::cargo_bin("migra")?
            .arg("-c")
            .arg(path_to_file("Migra_env.toml"))
            .arg("up")
            .assert()
            .success();

        let mut conn = postgres::Client::connect(DATABASE_URL_ENV_VALUE, postgres::NoTls)?;
        let res = conn.query("SELECT p.id, a.id FROM persons AS p, articles AS a", &[])?;

        assert_eq!(
            res.into_iter()
                .map(|row| (row.get(0), row.get(1)))
                .collect::<Vec<(i32, i32)>>(),
            Vec::new()
        );

        Command::cargo_bin("migra")?
            .arg("-c")
            .arg(path_to_file("Migra_env.toml"))
            .arg("down")
            .assert()
            .success();

        Command::cargo_bin("migra")?
            .arg("-c")
            .arg(path_to_file("Migra_env.toml"))
            .arg("down")
            .assert()
            .success();

        drop(env);

        Ok(())
    }
}
