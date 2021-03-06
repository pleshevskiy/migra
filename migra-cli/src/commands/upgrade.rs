use crate::database::migration::*;
use crate::database::transaction::with_transaction;
use crate::database::DatabaseConnectionManager;
use crate::opts::UpgradeCommandOpt;
use crate::Config;
use crate::StdResult;

pub(crate) fn upgrade_pending_migrations(config: Config, opts: UpgradeCommandOpt) -> StdResult<()> {
    let mut connection_manager = DatabaseConnectionManager::connect(&config.database)?;
    let conn = connection_manager.connection();

    let migration_manager = MigrationManager::new();

    let applied_migration_names = migration_manager.applied_migration_names(conn)?;
    let migrations = config.migrations()?;

    let pending_migrations = filter_pending_migrations(migrations, &applied_migration_names);
    if pending_migrations.is_empty() {
        println!("Up to date");
    } else if let Some(migration_name) = opts.migration_name {
        let target_migration = pending_migrations
            .iter()
            .find(|m| m.name() == &migration_name);
        match target_migration {
            Some(migration) => {
                print_migration_info(migration);
                with_transaction(conn, &mut |conn| migration_manager.upgrade(conn, migration))?;
            }
            None => {
                eprintln!(r#"Cannot find migration with "{}" name"#, migration_name);
            }
        }
    } else {
        let upgrade_migrations_number = opts
            .migrations_number
            .unwrap_or_else(|| pending_migrations.len());

        for migration in &pending_migrations[..upgrade_migrations_number] {
            print_migration_info(migration);
            with_transaction(conn, &mut |conn| migration_manager.upgrade(conn, migration))?;
        }
    }

    Ok(())
}

fn print_migration_info(migration: &Migration) {
    println!("upgrade {}...", migration.name());
}
