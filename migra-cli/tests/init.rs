mod common;
use common::*;
use std::fs;

#[test]
fn init_manifest_with_default_config() -> TestResult {
    Command::cargo_bin("migra")?
        .arg("init")
        .assert()
        .success()
        .stdout(contains("Created Migra.toml"));

    let content = fs::read_to_string("Migra.toml")?;

    assert_eq!(
        content,
        r#"root = "database"

[database]
connection = "$DATABASE_URL"
"#
    );

    fs::remove_file("Migra.toml")?;

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
