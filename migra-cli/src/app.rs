use crate::commands;
use crate::error::*;
use crate::opts::Command;
use crate::AppOpt;
use crate::Config;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Clone)]
pub(crate) struct App {
    app_opt: AppOpt,
}

impl App {
    pub fn new(app_opt: AppOpt) -> Self {
        App { app_opt }
    }

    pub fn config_path(&self) -> Option<&PathBuf> {
        self.app_opt.config_path.as_ref()
    }

    pub fn config(&self) -> MigraResult<Config> {
        Config::read(self.config_path())
    }

    pub fn run_command(&self) -> StdResult<()> {
        match self.app_opt.command.clone() {
            Command::Init => {
                commands::initialize_migra_manifest(self)?;
            }
            Command::Apply(cmd_opts) => {
                commands::apply_sql(self, cmd_opts)?;
            }
            Command::Make(cmd_opts) => {
                commands::make_migration(self, cmd_opts)?;
            }
            Command::List => {
                commands::print_migration_lists(self)?;
            }
            Command::Upgrade(cmd_opts) => {
                commands::upgrade_pending_migrations(self, cmd_opts)?;
            }
            Command::Downgrade(cmd_opts) => {
                commands::rollback_applied_migrations(self, cmd_opts)?;
            }
            Command::Completions(cmd_opts) => {
                AppOpt::clap().gen_completions_to(
                    env!("CARGO_BIN_NAME"),
                    cmd_opts.into(),
                    &mut std::io::stdout(),
                );
            }
        }

        Ok(())
    }
}
