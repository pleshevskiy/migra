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
            .stdout(contains(
                r#"Missed "DATABASE_URL" environment variable
No connection to database

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
