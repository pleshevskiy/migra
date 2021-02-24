use crate::config::Config;
use crate::database::prelude::*;
use crate::database::MigrationManager;
use crate::opts::DowngradeCommandOpt;
use crate::StdResult;
use std::cmp;
use std::convert::TryFrom;

pub(crate) fn rollback_applied_migrations(
    config: Config,
    opts: DowngradeCommandOpt,
) -> StdResult<()> {
    let mut manager = MigrationManager::try_from(&config)?;

    let applied_migrations = manager.applied_migration_names()?;
    let migrations = config.migrations()?;

    let rollback_migrations_number = if opts.all_migrations {
        applied_migrations.len()
    } else {
        cmp::min(opts.migrations_number, applied_migrations.len())
    };

    for migration_name in &applied_migrations[..rollback_migrations_number] {
        if let Some(migration) = migrations.iter().find(|m| m.name() == migration_name) {
            println!("downgrade {}...", migration.name());
            manager.downgrade(&migration)?;
        }
    }

    Ok(())
}
