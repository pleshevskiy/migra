use crate::app::App;
use crate::database;
use crate::opts::ApplyCommandOpt;

pub(crate) fn apply_sql(app: &App, cmd_opts: &ApplyCommandOpt) -> migra::StdResult<()> {
    let config = app.config()?;
    let mut client = database::create_client_from_config(&config)?;

    let file_contents = cmd_opts
        .file_paths
        .clone()
        .into_iter()
        .map(|file_path| {
            let mut file_path = config.directory_path().join(file_path);
            if file_path.extension().is_none() {
                file_path.set_extension("sql");
            }
            file_path
        })
        .map(std::fs::read_to_string)
        .collect::<Result<Vec<_>, _>>()?;

    database::run_in_transaction(&mut client, |client| {
        file_contents
            .iter()
            .try_for_each(|content| client.apply_sql(content))
            .map_err(From::from)
    })?;

    Ok(())
}
