#![deny(clippy::all, clippy::pedantic)]
#![forbid(unsafe_code)]

#[macro_use]
extern crate cfg_if;

#[cfg(not(any(feature = "postgres", feature = "mysql")))]
compile_error!(r#"Either features "postgres" or "mysql" must be enabled for "migra" crate"#);

mod app;
mod client;
mod commands;
mod config;
mod error;
pub use error::Error;

mod opts;

use crate::error::StdResult;
use app::App;
use config::Config;
use opts::{AppOpt, StructOpt};

fn main() -> StdResult<()> {
    #[cfg(feature = "dotenv")]
    dotenv::dotenv().ok();

    App::new(AppOpt::from_args()).run_command()
}
