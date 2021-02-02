#![deny(clippy::all)]

mod config;
mod opts;

use config::Config;
use migra_core::path::PathBuilder;
use opts::{AppOpt, ApplyCommandOpt, Command, StructOpt};
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
