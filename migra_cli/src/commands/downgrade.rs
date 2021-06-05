use crate::app::App;
use crate::client;
use crate::client::maybe_with_transaction;
use crate::opts::DowngradeCommandOpt;
use crate::StdResult;
use std::cmp;

pub(crate) fn rollback_applied_migrations(app: &App, opts: &DowngradeCommandOpt) -> StdResult<()> {
    let config = app.config()?;
    let mut client = client::create(
        &config.database.client(),
        &config.database.connection_string()?,
    )?;

    let applied_migrations = client.applied_migrations()?;
    let all_migrations = migra::fs::get_all_migrations(&config.migration_dir_path())?;

    let rollback_migrations_number = if opts.all_migrations {
        applied_migrations.len()
    } else {
        cmp::min(opts.migrations_number, applied_migrations.len())
    };

    maybe_with_transaction(
        opts.transaction_opts.single_transaction,
        &mut client,
        &mut |mut client| {
            applied_migrations[..rollback_migrations_number]
                .iter()
                .try_for_each(|applied_migration| {
                    if all_migrations.contains(applied_migration) {
                        println!("downgrade {}...", applied_migration.name());
                        maybe_with_transaction(
                            !opts.transaction_opts.single_transaction,
                            &mut client,
                            &mut |client| client.apply_downgrade_migration(&applied_migration),
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
