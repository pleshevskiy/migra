use crate::app::App;
use crate::database;
use crate::opts::DowngradeCommandOpt;
use std::cmp;

pub(crate) fn rollback_applied_migrations(
    app: &App,
    opts: &DowngradeCommandOpt,
) -> migra::StdResult<()> {
    let config = app.config()?;
    let mut client = database::create_client_from_config(&config)?;

    client.create_migrations_table()?;

    let applied_migrations =
        client.get_extended_applied_migrations(&config.migration_dir_path())?;
    let all_migrations = migra::fs::get_all_migrations(&config.migration_dir_path())?;

    let rollback_migrations_number = if opts.all_migrations {
        applied_migrations.len()
    } else {
        cmp::min(opts.migrations_number, applied_migrations.len())
    };

    dbg!(&rollback_migrations_number);

    database::maybe_with_transaction(
        opts.transaction_opts.single_transaction,
        &mut client,
        &mut |mut client| {
            applied_migrations[..rollback_migrations_number]
                .iter()
                .try_for_each(|applied_migration| {
                    if all_migrations.contains_name(applied_migration.name()) {
                        println!("downgrade {}...", applied_migration.name());
                        database::maybe_with_transaction(
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
