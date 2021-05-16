use crate::app::App;
use crate::database::migration::*;
use crate::database::transaction::maybe_with_transaction;
use crate::database::DatabaseConnectionManager;
use crate::opts::UpgradeCommandOpt;
use crate::StdResult;

pub(crate) fn upgrade_pending_migrations(app: &App, opts: UpgradeCommandOpt) -> StdResult<()> {
    let config = app.config()?;
    let mut connection_manager = DatabaseConnectionManager::connect(&config.database)?;
    let conn = connection_manager.connection();

    let migration_manager = MigrationManager::from(&config);

    let applied_migration_names = migration_manager.applied_migration_names(conn)?;
    let migrations = config.migrations()?;

    let pending_migrations = filter_pending_migrations(migrations, &applied_migration_names);
    if pending_migrations.is_empty() {
        println!("Up to date");
        return Ok(());
    }

    let migrations: Vec<Migration> = if let Some(migration_name) = opts.migration_name.clone() {
        let target_migration = pending_migrations
            .into_iter()
            .find(|m| m.name() == &migration_name);
        match target_migration {
            Some(migration) => vec![migration],
            None => {
                eprintln!(r#"Cannot find migration with "{}" name"#, migration_name);
                return Ok(());
            }
        }
    } else {
        let upgrade_migrations_number = opts
            .migrations_number
            .unwrap_or_else(|| pending_migrations.len());

        pending_migrations[..upgrade_migrations_number].to_vec()
    };

    maybe_with_transaction(
        opts.transaction_opts.single_transaction,
        conn,
        &mut |conn| {
            migrations
                .iter()
                .try_for_each(|migration| {
                    print_migration_info(migration);
                    maybe_with_transaction(
                        !opts.transaction_opts.single_transaction,
                        conn,
                        &mut |conn| migration_manager.upgrade(conn, migration),
                    )
                })
                .map_err(From::from)
        },
    )?;

    Ok(())
}

fn print_migration_info(migration: &Migration) {
    println!("upgrade {}...", migration.name());
}
