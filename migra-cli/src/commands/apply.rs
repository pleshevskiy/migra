use crate::config::Config;
use crate::databases::*;
use crate::migration::{DatabaseMigrationManager, MigrationManager};
use crate::opts::ApplyCommandOpt;
use crate::path::PathBuilder;
use crate::StdResult;
use std::convert::TryFrom;

pub(crate) fn apply_sql(config: Config, opts: ApplyCommandOpt) -> StdResult<()> {
    let connection = PostgresConnection::try_from(&config)?;
    let mut manager = MigrationManager::new(connection);

    let file_path = PathBuilder::from(config.directory_path())
        .append(opts.file_name)
        .default_extension("sql")
        .build();

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
