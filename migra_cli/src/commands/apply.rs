use crate::app::App;
use crate::client::maybe_with_transaction;
use crate::opts::ApplyCommandOpt;

pub(crate) fn apply_sql(app: &App, cmd_opts: &ApplyCommandOpt) -> migra::StdResult<()> {
    let config = app.config()?;
    let mut client = crate::client::create(
        &config.database.client(),
        &config.database.connection_string()?,
        &config.migrations.table_name(),
    )?;

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

    maybe_with_transaction(
        cmd_opts.transaction_opts.single_transaction,
        &mut client,
        &mut |mut client| {
            file_contents
                .iter()
                .try_for_each(|content| {
                    maybe_with_transaction(
                        !cmd_opts.transaction_opts.single_transaction,
                        &mut client,
                        &mut |client| client.apply_sql(content),
                    )
                })
                .map_err(From::from)
        },
    )?;

    Ok(())
}
