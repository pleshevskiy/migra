use crate::app::App;
use crate::database;
use crate::opts::UpgradeCommandOpt;
use migra::migration;
use migra::should_run_in_transaction;

pub(crate) fn upgrade_pending_migrations(
    app: &App,
    opts: &UpgradeCommandOpt,
) -> migra::StdResult<()> {
    let config = app.config()?;
    let mut client = database::create_client_from_config(&config)?;

    client.create_migrations_table()?;

    let migrations_dir_path = config.migration_dir_path();
    let applied_migration_names = client.get_applied_migrations()?;
    let all_migrations = migra::fs::get_all_migrations(&migrations_dir_path)?;

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
        if let Some(migration) = target_migration {
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

    let migrations_with_content = migrations
        .iter()
        .map(|migration| {
            let migration_name = migration.name();
            let migration_file_path = migrations_dir_path.join(migration_name).join("up.sql");
            std::fs::read_to_string(migration_file_path).map(|content| (migration_name, content))
        })
        .collect::<Result<Vec<_>, _>>()?;

    should_run_in_transaction(
        &mut client,
        opts.transaction_opts.single_transaction,
        |client| {
            migrations_with_content
                .iter()
                .try_for_each(|(migration_name, content)| {
                    println!("upgrade {}...", migration_name);
                    should_run_in_transaction(
                        client,
                        !opts.transaction_opts.single_transaction,
                        |client| client.run_upgrade_migration(migration_name, &content),
                    )
                })
                .map_err(From::from)
        },
    )?;

    Ok(())
}
