#![deny(clippy::all)]

mod config;
mod database;
mod error;
mod migration;
mod opts;
mod path;

use chrono::Local;
use config::Config;
use error::ErrorKind;
use opts::{AppOpt, ApplyCommandOpt, Command, MakeCommandOpt, StructOpt};
use path::PathBuilder;
use std::fs;

const EM_DASH: char = '—';

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = AppOpt::from_args();

    match opt.command {
        Command::Init => {
            Config::initialize(opt.config)?;
        }
        Command::Apply(ApplyCommandOpt { file_name }) => {
            let config = Config::read(opt.config)?;

            let database_connection_string = &config.database_connection_string()?;
            let mut client = database::connect(database_connection_string)?;

            let file_path = PathBuilder::from(config.directory_path())
                .append(file_name)
                .default_extension("sql")
                .build();

            let content = fs::read_to_string(file_path)?;

            match database::apply_sql(&mut client, &content) {
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

            let migration_dir_path = PathBuilder::from(config.migration_dir_path())
                .append(format!("{}_{}", now, migration_name))
                .build();
            if !migration_dir_path.exists() {
                fs::create_dir_all(&migration_dir_path)?;
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

            let applied_migrations = match config.database_connection_string() {
                Ok(ref database_connection_string) => {
                    let mut client = database::connect(database_connection_string)?;
                    let applied_migrations = database::applied_migrations(&mut client)?;

                    println!("Applied migrations:");
                    if applied_migrations.is_empty() {
                        println!("{}", EM_DASH);
                    } else {
                        applied_migrations
                            .iter()
                            .rev()
                            .for_each(|name| println!("{}", name));
                    }

                    applied_migrations
                }
                Err(e) if *e.kind() == ErrorKind::MissedEnvVar(String::new()) => {
                    println!("{}", e.kind());
                    println!("No connection to database");

                    Vec::new()
                }
                Err(e) => panic!(e),
            };

            println!();

            let pending_migrations = config
                .migrations()?
                .into_iter()
                .filter(|m| !applied_migrations.contains(m.name()))
                .collect::<Vec<_>>();
            println!("Pending migrations:");
            if pending_migrations.is_empty() {
                println!("{}", EM_DASH);
            } else {
                pending_migrations.iter().for_each(|m| {
                    println!("{}", m.name());
                });
            }
        }
        Command::Upgrade => {
            let config = Config::read(opt.config)?;

            let database_connection_string = &config.database_connection_string()?;
            let mut client = database::connect(database_connection_string)?;

            let applied_migrations = database::applied_migrations(&mut client)?;

            let migrations = config.migrations()?;

            if migrations.is_empty()
                || migrations.last().map(|m| m.name()) == applied_migrations.first()
            {
                println!("Up to date");
            } else {
                for m in migrations
                    .iter()
                    .filter(|m| !applied_migrations.contains(m.name()))
                {
                    println!("upgrade {}...", m.name());
                    m.upgrade(&mut client)?;
                }
            }
        }
        Command::Downgrade => {
            let config = Config::read(opt.config)?;

            let database_connection_string = &config.database_connection_string()?;
            let mut client = database::connect(database_connection_string)?;

            let applied_migrations = database::applied_migrations(&mut client)?;
            let migrations = config.migrations()?;

            if let Some(first_applied_migration) = applied_migrations.first() {
                if let Some(migration) = migrations
                    .iter()
                    .find(|m| m.name() == first_applied_migration)
                {
                    println!("downgrade {}...", migration.name());
                    migration.downgrade(&mut client)?;
                }
            }
        }
    }

    Ok(())
}
