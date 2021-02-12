use crate::config::Config;
use crate::database;
use crate::StdResult;

pub(crate) fn downgrade_applied_migrations(config: Config) -> StdResult<()> {
    let database_connection_string = &config.database_connection_string()?;
    let mut client = database::connect(database_connection_string)?;

    let applied_migrations = database::applied_migrations(&mut client)?;
    let migrations = config.migrations()?;

    if let Some(first_applied_migration) = applied_migrations.first() {
        if let Some(migration) = migrations
            .iter()
            .find(|m| m.name() == first_applied_migration)
        {
            println!("downgrade {}...", migration.name());
            migration.downgrade(&mut client)?;
        }
    }

    Ok(())
}
