#![deny(clippy::all)]

mod config;
mod opts;

use config::Config;
use opts::{StructOpt, AppOpt, ApplyOpt};
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = AppOpt::from_args();

    match opt {
        AppOpt::Init => {
            Config::initialize()?;
        },
        AppOpt::Apply(ApplyOpt { file_name }) => {
            let config = Config::read();

            let mut client = migra_core::database::connect(&config.database.connection)?;

            let file_name = Path::new(&file_name);
            let mut filepath = PathBuf::from(&config.directory);
            filepath.push(file_name);
            if file_name.extension().is_none() {
                filepath.set_extension("sql");
            }

            let content = std::fs::read_to_string(filepath)?;

            match migra_core::database::apply_sql(&mut client, &content) {
                Ok(_) => {
                    println!("File was applied successfully")
                }
                Err(err) => {
                    println!("{}", err)
                }
            }
        }
    }

    Ok(())
}
