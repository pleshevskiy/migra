use crate::app::App;
use crate::opts::MakeCommandOpt;
use chrono::Local;
use std::fs;

pub(crate) fn make_migration(app: &App, opts: &MakeCommandOpt) -> migra::StdResult<()> {
    let config = app.config()?;
    let date_format = config.migrations.date_format();
    let formatted_current_timestamp = Local::now().format(&date_format);

    let migration_name: String = opts
        .migration_name
        .to_lowercase()
        .chars()
        .map(|c| match c {
            '0'..='9' | 'a'..='z' => c,
            _ => '_',
        })
        .collect();

    let migration_dir_path = config.migration_dir_path().join(format!(
        "{}_{}",
        formatted_current_timestamp, migration_name
    ));
    if !migration_dir_path.exists() {
        fs::create_dir_all(&migration_dir_path)?;
    }

    let upgrade_migration_path = &migration_dir_path.join("up.sql");
    if !upgrade_migration_path.exists() {
        fs::write(upgrade_migration_path, "-- Your SQL goes here\n\n")?;
    }

    let downgrade_migration_path = &migration_dir_path.join("down.sql");
    if !downgrade_migration_path.exists() {
        fs::write(
            downgrade_migration_path,
            "-- This file should undo anything in `up.sql`\n\n",
        )?;
    }

    println!(
        "Structure for migration has been created in the {}",
        migration_dir_path.to_str().unwrap()
    );

    Ok(())
}
