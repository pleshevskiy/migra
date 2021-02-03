#![deny(clippy::all)]

mod config;
mod opts;

use chrono::Local;
use config::Config;
use migra_core::path::PathBuilder;
use opts::{AppOpt, ApplyCommandOpt, Command, MakeCommandOpt, StructOpt};
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = AppOpt::from_args();

    match opt.command {
        Command::Init => {
            Config::initialize()?;
        }
        Command::Apply(ApplyCommandOpt { file_name }) => {
            let config = Config::read(opt.config)?;

            let mut client = migra_core::database::connect(&config.database.connection)?;

            let file_path = PathBuilder::from(config.directory_path())
                .append(file_name)
                .default_extension("sql")
                .build();

            let content = fs::read_to_string(file_path)?;

            match migra_core::database::apply_sql(&mut client, &content) {
                Ok(_) => {
                    println!("File was applied successfully")
                }
                Err(err) => {
                    println!("{}", err)
                }
            }
        }
        Command::Make(MakeCommandOpt { migration_name }) => {
            let config = Config::read(opt.config)?;

            let now = Local::now().format("%y%m%d%H%M%S");

            let migration_name: String = migration_name
                .to_lowercase()
                .chars()
                .map(|c| match c {
                    '0'..='9' | 'a'..='z' => c,
                    _ => '_',
                })
                .collect();

            let migration_dir_path = PathBuilder::from(config.directory_path())
                .append(format!("{}_{}", now, migration_name))
                .build();
            if !migration_dir_path.exists() {
                fs::create_dir(&migration_dir_path)?;
            }

            let upgrade_migration_path = PathBuilder::from(&migration_dir_path)
                .append("up.sql")
                .build();
            if !upgrade_migration_path.exists() {
                fs::write(upgrade_migration_path, "-- Your SQL goes here\n\n")?;
            }

            let downgrade_migration_path = PathBuilder::from(&migration_dir_path)
                .append("down.sql")
                .build();
            if !downgrade_migration_path.exists() {
                fs::write(
                    downgrade_migration_path,
                    "-- This file should undo anything in `up.sql`\n\n",
                )?;
            }
        }
        Command::List => {
            let config = Config::read(opt.config)?;

            let migration_dirs = config.migration_dirs()?;
            if migration_dirs.is_empty() {
                println!(
                    "You haven't migrations in {}",
                    config.directory_path().to_str().unwrap()
                );
            } else {
                migration_dirs.iter().for_each(|dir| {
                    let file_name = dir.file_name().and_then(|name| name.to_str()).unwrap();
                    println!("{}", file_name);
                });
            }
        }
        Command::Upgrade | Command::Downgrade => {
            unimplemented!();
        }
    }

    Ok(())
}
