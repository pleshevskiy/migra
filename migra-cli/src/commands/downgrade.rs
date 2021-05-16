use crate::app::App;
use crate::database::prelude::*;
use crate::database::transaction::maybe_with_transaction;
use crate::database::{DatabaseConnectionManager, MigrationManager};
use crate::opts::DowngradeCommandOpt;
use crate::StdResult;
use std::cmp;

pub(crate) fn rollback_applied_migrations(app: &App, opts: DowngradeCommandOpt) -> StdResult<()> {
    let config = app.config()?;
    let mut connection_manager = DatabaseConnectionManager::connect(&config.database)?;
    let conn = connection_manager.connection();
    let migration_manager = MigrationManager::from(&config);

    let applied_migrations = migration_manager.applied_migration_names(conn)?;
    let migrations = config.migrations()?;

    let rollback_migrations_number = if opts.all_migrations {
        applied_migrations.len()
    } else {
        cmp::min(opts.migrations_number, applied_migrations.len())
    };

    maybe_with_transaction(
        opts.transaction_opts.single_transaction,
        conn,
        &mut |conn| {
            applied_migrations[..rollback_migrations_number]
                .iter()
                .try_for_each(|migration_name| {
                    if let Some(migration) = migrations.iter().find(|m| m.name() == migration_name)
                    {
                        println!("downgrade {}...", migration.name());
                        maybe_with_transaction(
                            !opts.transaction_opts.single_transaction,
                            conn,
                            &mut |conn| migration_manager.downgrade(conn, &migration),
                        )
                    } else {
                        Ok(())
                    }
                })
                .map_err(From::from)
        },
    )?;

    Ok(())
}
