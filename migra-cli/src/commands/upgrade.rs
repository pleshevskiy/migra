use crate::database::DatabaseConnection;
use crate::migration::Migration;
use crate::Config;
use crate::StdResult;
use std::convert::TryFrom;

pub(crate) fn upgrade_pending_migrations(config: Config) -> StdResult<()> {
    let mut connection = DatabaseConnection::try_from(&config)?;

    let applied_migration_names = connection.applied_migration_names()?;
    let migrations = config.migrations()?;

    if is_up_to_date_migrations(&migrations, &applied_migration_names) {
        println!("Up to date");
    } else {
        let pending_migrations = filter_pending_migrations(migrations, &applied_migration_names);

        for migration in pending_migrations.iter() {
            println!("upgrade {}...", migration.name());
            migration.upgrade(&mut connection)?;
        }
    }

    Ok(())
}

fn is_up_to_date_migrations(migrations: &[Migration], applied_migration_names: &[String]) -> bool {
    migrations.is_empty() || migrations.last().map(|m| m.name()) == applied_migration_names.first()
}

fn filter_pending_migrations(
    migrations: Vec<Migration>,
    applied_migration_names: &[String],
) -> Vec<Migration> {
    migrations
        .into_iter()
        .filter(|m| !applied_migration_names.contains(m.name()))
        .collect()
}
