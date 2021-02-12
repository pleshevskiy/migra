use crate::database;
use crate::Config;
use crate::StdResult;

pub(crate) fn upgrade_pending_migrations(config: Config) -> StdResult<()> {
    let database_connection_string = &config.database_connection_string()?;
    let mut client = database::connect(database_connection_string)?;

    let applied_migrations = database::applied_migrations(&mut client)?;

    let migrations = config.migrations()?;

    if migrations.is_empty() || migrations.last().map(|m| m.name()) == applied_migrations.first() {
        println!("Up to date");
    } else {
        for m in migrations
            .iter()
            .filter(|m| !applied_migrations.contains(m.name()))
        {
            println!("upgrade {}...", m.name());
            m.upgrade(&mut client)?;
        }
    }

    Ok(())
}
