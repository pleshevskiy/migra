use crate::config::Config;
use crate::database::prelude::*;
use crate::database::MigrationManager;
use crate::opts::ApplyCommandOpt;
use crate::StdResult;
use std::convert::TryFrom;

pub(crate) fn apply_sql(config: Config, opts: ApplyCommandOpt) -> StdResult<()> {
    let mut manager = MigrationManager::try_from(&config)?;

    let file_path = {
        let mut file_path = config.directory_path().join(opts.file_name);
        if file_path.extension().is_none() {
            file_path.set_extension("sql");
        }
        file_path
    };

    let content = std::fs::read_to_string(file_path)?;

    match manager.apply_sql(&content) {
        Ok(_) => {
            println!("File was applied successfully");
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    Ok(())
}
