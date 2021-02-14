use crate::config::Config;
use crate::database::PostgresConnection;
use crate::opts::ApplyCommandOpt;
use crate::path::PathBuilder;
use crate::StdResult;
use std::convert::TryFrom;

pub(crate) fn apply_sql(config: Config, opts: ApplyCommandOpt) -> StdResult<()> {
    let mut connection = PostgresConnection::try_from(&config)?;

    let file_path = PathBuilder::from(config.directory_path())
        .append(opts.file_name)
        .default_extension("sql")
        .build();

    let content = std::fs::read_to_string(file_path)?;

    match connection.apply_sql(&content) {
        Ok(_) => {
            println!("File was applied successfully");
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    Ok(())
}
