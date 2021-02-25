#![deny(clippy::all)]
#![forbid(unsafe_code)]

mod commands;
mod config;
mod database;
mod error;
mod opts;

use crate::error::StdResult;
use config::Config;
use opts::{AppOpt, Command, StructOpt};
use std::io;

fn main() -> StdResult<()> {
    #[cfg(feature = "dotenv")]
    dotenv::dotenv().ok();

    let opt = AppOpt::from_args();

    match opt.command {
        Command::Init => {
            commands::initialize_migra_manifest(opt.config)?;
        }
        Command::Apply(opts) => {
            let config = Config::read(opt.config)?;
            commands::apply_sql(config, opts)?;
        }
        Command::Make(opts) => {
            let config = Config::read(opt.config)?;
            commands::make_migration(config, opts)?;
        }
        Command::List => {
            let config = Config::read(opt.config)?;
            commands::print_migration_lists(config)?;
        }
        Command::Upgrade(opts) => {
            let config = Config::read(opt.config)?;
            commands::upgrade_pending_migrations(config, opts)?;
        }
        Command::Downgrade(opts) => {
            let config = Config::read(opt.config)?;
            commands::rollback_applied_migrations(config, opts)?;
        }
        Command::Completions(opts) => {
            AppOpt::clap().gen_completions_to(
                env!("CARGO_BIN_NAME"),
                opts.into(),
                &mut io::stdout(),
            );
        }
    }

    Ok(())
}
