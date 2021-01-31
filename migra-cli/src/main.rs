#![deny(clippy::all)]

mod config;
mod opts;

use config::Config;
use opts::{AppOpt, StructOpt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = AppOpt::from_args();
    dbg!(&opt);

    let config = Config::read();
    dbg!(&config);

    match opt {
        AppOpt::Init => {
            Config::initialize()?;
        }
    }

    Ok(())
}
