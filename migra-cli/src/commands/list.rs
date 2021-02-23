use crate::config::Config;
use crate::database::migration::filter_pending_migrations;
use crate::database::prelude::*;
use crate::database::{DatabaseConnectionManager, Migration, MigrationManager};
use crate::error::{Error, StdResult};

const EM_DASH: char = '—';

pub(crate) fn print_migration_lists(config: Config) -> StdResult<()> {
    let applied_migration_names = match config.database.connection_string() {
        Ok(ref database_connection_string) => {
            let connection_manager = DatabaseConnectionManager::new(&config.database);
            let conn = connection_manager.connect_with_string(database_connection_string)?;
            let mut manager = MigrationManager::new(conn);

            let applied_migration_names = manager.applied_migration_names()?;

            show_applied_migrations(&applied_migration_names);

            applied_migration_names
        }
        Err(e) if e == Error::MissedEnvVar(String::new()) => {
            eprintln!("WARNING: {}", e);
            eprintln!("WARNING: No connection to database");

            Vec::new()
        }
        Err(e) => panic!("{}", e),
    };

    println!();

    let pending_migrations =
        filter_pending_migrations(config.migrations()?, &applied_migration_names);
    show_pending_migrations(&pending_migrations);

    Ok(())
}

fn show_applied_migrations(applied_migration_names: &[String]) {
    println!("Applied migrations:");
    if applied_migration_names.is_empty() {
        println!("{}", EM_DASH);
    } else {
        applied_migration_names
            .iter()
            .rev()
            .for_each(|name| println!("{}", name));
    }
}

fn show_pending_migrations(pending_migrations: &[Migration]) {
    println!("Pending migrations:");
    if pending_migrations.is_empty() {
        println!("{}", EM_DASH);
    } else {
        pending_migrations.iter().for_each(|m| {
            println!("{}", m.name());
        });
    }
}
