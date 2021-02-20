mod common;

use common::*;
use std::io::Write;

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
