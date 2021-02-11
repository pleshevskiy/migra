mod common;

use std::io::Write;
use common::*;

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
