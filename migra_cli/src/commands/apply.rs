use crate::app::App;
use crate::database::prelude::*;
use crate::database::transaction::maybe_with_transaction;
use crate::database::{DatabaseConnectionManager, MigrationManager};
use crate::opts::ApplyCommandOpt;
use crate::StdResult;

pub(crate) fn apply_sql(app: &App, cmd_opts: ApplyCommandOpt) -> StdResult<()> {
    let config = app.config()?;
    let mut connection_manager = DatabaseConnectionManager::connect(&config.database)?;
    let conn = connection_manager.connection();

    let migration_manager = MigrationManager::from(&config);

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
        conn,
        &mut |conn| {
            file_contents
                .iter()
                .try_for_each(|content| {
                    maybe_with_transaction(
                        !cmd_opts.transaction_opts.single_transaction,
                        conn,
                        &mut |conn| migration_manager.apply_sql(conn, content),
                    )
                })
                .map_err(From::from)
        },
    )?;

    Ok(())
}
