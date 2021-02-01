#![deny(clippy::all)]

mod config;
mod opts;

use config::Config;
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

            let file_path = migra_core::path::PathBuilder::from(config.root)
                .append(config.directory)
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
        Command::List | Command::Upgrade | Command::Downgrade => {
            unimplemented!();
        }
    }

    Ok(())
}
