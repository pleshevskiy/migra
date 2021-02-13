use crate::config::Config;
use crate::database::DatabaseConnection;
use crate::migration::Downgrade;
use crate::StdResult;
use std::convert::TryFrom;

pub(crate) fn downgrade_applied_migrations(config: Config) -> StdResult<()> {
    let mut connection = DatabaseConnection::try_from(&config)?;

    let applied_migrations = connection.applied_migration_names()?;
    let migrations = config.migrations()?;

    if let Some(first_applied_migration) = applied_migrations.first() {
        if let Some(migration) = migrations
            .iter()
            .find(|m| m.name() == first_applied_migration)
        {
            println!("downgrade {}...", migration.name());
            migration.downgrade(&mut connection)?;
        }
    }

    Ok(())
}
