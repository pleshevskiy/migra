use crate::opts::MakeCommandOpt;
use crate::path::PathBuilder;
use crate::Config;
use crate::StdResult;
use chrono::Local;

pub(crate) fn make_migration(config: Config, opts: MakeCommandOpt) -> StdResult<()> {
    let now = Local::now().format("%y%m%d%H%M%S");

    let migration_name: String = opts
        .migration_name
        .to_lowercase()
        .chars()
        .map(|c| match c {
            '0'..='9' | 'a'..='z' => c,
            _ => '_',
        })
        .collect();

    let migration_dir_path = PathBuilder::from(config.migration_dir_path())
        .append(format!("{}_{}", now, migration_name))
        .build();
    if !migration_dir_path.exists() {
        std::fs::create_dir_all(&migration_dir_path)?;
    }

    let upgrade_migration_path = PathBuilder::from(&migration_dir_path)
        .append("up.sql")
        .build();
    if !upgrade_migration_path.exists() {
        std::fs::write(upgrade_migration_path, "-- Your SQL goes here\n\n")?;
    }

    let downgrade_migration_path = PathBuilder::from(&migration_dir_path)
        .append("down.sql")
        .build();
    if !downgrade_migration_path.exists() {
        std::fs::write(
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
