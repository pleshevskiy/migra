use crate::config::Config;
use crate::database::prelude::*;
use crate::database::MigrationManager;
use crate::StdResult;
use std::convert::TryFrom;

pub(crate) fn downgrade_applied_migrations(config: Config) -> StdResult<()> {
    let mut manager = MigrationManager::try_from(&config)?;

    let applied_migrations = manager.applied_migration_names()?;
    let migrations = config.migrations()?;

    if let Some(first_applied_migration) = applied_migrations.first() {
        if let Some(migration) = migrations
            .iter()
            .find(|m| m.name() == first_applied_migration)
        {
            println!("downgrade {}...", migration.name());
            manager.downgrade(&migration)?;
        }
    }

    Ok(())
}
