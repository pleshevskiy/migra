use crate::app::App;
use crate::database;
use crate::opts::UpgradeCommandOpt;
use migra::migration;

pub(crate) fn upgrade_pending_migrations(
    app: &App,
    opts: &UpgradeCommandOpt,
) -> migra::StdResult<()> {
    let config = app.config()?;
    let mut client = database::create_client_from_config(&config)?;

    client.create_migrations_table()?;

    let applied_migration_names =
        client.get_extended_applied_migrations(&config.migration_dir_path())?;
    let all_migrations = migra::fs::get_all_migrations(&config.migration_dir_path())?;

    let pending_migrations =
        migra::fs::filter_pending_migrations(&all_migrations, &applied_migration_names);
    if pending_migrations.is_empty() {
        println!("Up to date");
        return Ok(());
    }

    let migrations: migration::List = if let Some(migration_name) = opts.migration_name.clone() {
        let target_migration = (*pending_migrations)
            .clone()
            .into_iter()
            .find(|m| m.name() == &migration_name);
        if let Some(migration) = target_migration.clone() {
            vec![migration].into()
        } else {
            eprintln!(r#"Cannot find migration with "{}" name"#, migration_name);
            return Ok(());
        }
    } else {
        let upgrade_migrations_number = opts
            .migrations_number
            .unwrap_or_else(|| pending_migrations.len());

        pending_migrations[..upgrade_migrations_number]
            .to_vec()
            .into()
    };

    database::maybe_with_transaction(
        opts.transaction_opts.single_transaction,
        &mut client,
        &mut |mut client| {
            migrations
                .iter()
                .try_for_each(|migration| {
                    print_migration_info(migration);
                    database::maybe_with_transaction(
                        !opts.transaction_opts.single_transaction,
                        &mut client,
                        &mut |client| client.apply_upgrade_migration(migration),
                    )
                })
                .map_err(From::from)
        },
    )?;

    Ok(())
}

fn print_migration_info(migration: &migra::Migration) {
    println!("upgrade {}...", migration.name());
}
