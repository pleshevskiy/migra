use crate::config::Config;
use crate::database;
use crate::opts::ApplyCommandOpt;
use crate::path::PathBuilder;
use crate::StdResult;

pub(crate) fn apply_sql(config: Config, opts: ApplyCommandOpt) -> StdResult<()> {
    let database_connection_string = &config.database_connection_string()?;
    let mut client = database::connect(database_connection_string)?;

    let file_path = PathBuilder::from(config.directory_path())
        .append(opts.file_name)
        .default_extension("sql")
        .build();

    let content = std::fs::read_to_string(file_path)?;

    match database::apply_sql(&mut client, &content) {
        Ok(_) => {
            println!("File was applied successfully");
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    Ok(())
}
