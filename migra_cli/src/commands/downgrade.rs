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

    let migrations_dir_path = config.migration_dir_path();
    let applied_migrations = client.get_applied_migrations()?;
    let all_migrations = migra::fs::get_all_migrations(&migrations_dir_path)?;

    let rollback_migrations_number = if opts.all_migrations {
        applied_migrations.len()
    } else {
        cmp::min(opts.migrations_number, applied_migrations.len())
    };

    let migrations = applied_migrations[..rollback_migrations_number].to_vec();
    let migrations_with_content = migrations
        .iter()
        .map(|migration| {
            let migration_name = migration.name();
            let migration_file_path = migrations_dir_path.join(migration_name).join("down.sql");
            std::fs::read_to_string(migration_file_path).map(|content| (migration_name, content))
        })
        .collect::<Result<Vec<_>, _>>()?;

    database::maybe_with_transaction(
        opts.transaction_opts.single_transaction,
        &mut client,
        &mut |mut client| {
            migrations_with_content
                .iter()
                .try_for_each(|(migration_name, content)| {
                    if all_migrations.contains_name(migration_name) {
                        println!("downgrade {}...", migration_name);
                        database::maybe_with_transaction(
                            !opts.transaction_opts.single_transaction,
                            &mut client,
                            &mut |client| client.run_downgrade_migration(migration_name, &content),
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
