use crate::database::{DatabaseConnection, PostgresConnection};
use crate::migration::Migration;
use crate::migration::{filter_pending_migrations, DatabaseMigrationManager, MigrationManager};
use crate::Config;
use crate::StdResult;
use std::convert::TryFrom;

pub(crate) fn upgrade_pending_migrations(config: Config) -> StdResult<()> {
    let mut manager = MigrationManager::new(PostgresConnection::try_from(&config)?);

    let applied_migration_names = manager.applied_migration_names()?;
    let migrations = config.migrations()?;

    if is_up_to_date_migrations(&migrations, &applied_migration_names) {
        println!("Up to date");
    } else {
        let pending_migrations = filter_pending_migrations(migrations, &applied_migration_names);
        upgrade_all_pending_migrations(manager, &pending_migrations)?;
    }

    Ok(())
}

fn is_up_to_date_migrations(migrations: &[Migration], applied_migration_names: &[String]) -> bool {
    migrations.is_empty() || migrations.last().map(|m| m.name()) == applied_migration_names.first()
}

fn upgrade_all_pending_migrations<Conn, ManagerT>(
    mut manager: ManagerT,
    pending_migrations: &[Migration],
) -> StdResult<()>
where
    Conn: DatabaseConnection,
    ManagerT: Sized + DatabaseMigrationManager<Conn>,
{
    for migration in pending_migrations.iter() {
        println!("upgrade {}...", migration.name());
        manager.upgrade(migration)?;
    }

    Ok(())
}
