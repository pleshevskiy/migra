use crate::database::migration::*;
use crate::opts::UpgradeCommandOpt;
use crate::Config;
use crate::StdResult;
use std::convert::TryFrom;

pub(crate) fn upgrade_pending_migrations(config: Config, opts: UpgradeCommandOpt) -> StdResult<()> {
    let mut manager = MigrationManager::try_from(&config)?;

    let applied_migration_names = manager.applied_migration_names()?;
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
                manager.upgrade(migration)?;
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
            manager.upgrade(migration)?;
        }
    }

    Ok(())
}

fn print_migration_info(migration: &Migration) {
    println!("upgrade {}...", migration.name());
}
