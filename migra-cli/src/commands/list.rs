use crate::config::Config;
use crate::database::DatabaseConnection;
use crate::error::{ErrorKind, StdResult};

const EM_DASH: char = '—';

pub(crate) fn print_migration_lists(config: Config) -> StdResult<()> {
    let applied_migrations = match config.database_connection_string() {
        Ok(ref database_connection_string) => {
            let mut connection = DatabaseConnection::connect(database_connection_string)?;
            let applied_migrations = connection.applied_migration_names()?;

            println!("Applied migrations:");
            if applied_migrations.is_empty() {
                println!("{}", EM_DASH);
            } else {
                applied_migrations
                    .iter()
                    .rev()
                    .for_each(|name| println!("{}", name));
            }

            applied_migrations
        }
        Err(e) if *e.kind() == ErrorKind::MissedEnvVar(String::new()) => {
            println!("{}", e.kind());
            println!("No connection to database");

            Vec::new()
        }
        Err(e) => panic!(e),
    };

    println!();

    let pending_migrations = config
        .migrations()?
        .into_iter()
        .filter(|m| !applied_migrations.contains(m.name()))
        .collect::<Vec<_>>();
    println!("Pending migrations:");
    if pending_migrations.is_empty() {
        println!("{}", EM_DASH);
    } else {
        pending_migrations.iter().for_each(|m| {
            println!("{}", m.name());
        });
    }

    Ok(())
}
