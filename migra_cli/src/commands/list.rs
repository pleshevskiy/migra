use crate::app::App;
use crate::database;
use crate::error::Error;
use migra::migration;

const EM_DASH: char = '—';

pub(crate) fn print_migration_lists(app: &App) -> migra::StdResult<()> {
    let config = app.config()?;
    let applied_migrations = match config.database.connection_string() {
        Ok(ref database_connection_string) => {
            let mut client = database::create_client(
                &config.database.client(),
                database_connection_string,
                &config.migrations.table_name(),
            )?;
            let applied_migrations = client.get_applied_migrations().unwrap_or_else(|err| {
                dbg!(err);
                migration::List::new()
            });

            show_applied_migrations(&applied_migrations);

            applied_migrations
        }
        Err(e) if e == Error::MissedEnvVar(String::new()) => {
            eprintln!("WARNING: {}", e);
            eprintln!("WARNING: No connection to database");

            migration::List::new()
        }
        Err(e) => panic!("{}", e),
    };

    println!();

    let all_migrations = migra::fs::get_all_migrations(&config.migration_dir_path())?;
    let pending_migrations = all_migrations.exclude(&applied_migrations);

    show_pending_migrations(&pending_migrations);

    Ok(())
}

fn show_applied_migrations(applied_migrations: &migration::List) {
    println!("Applied migrations:");
    if applied_migrations.is_empty() {
        println!("{}", EM_DASH);
    } else {
        applied_migrations
            .iter()
            .rev()
            .for_each(|migration| println!("{}", migration.name()));
    }
}

fn show_pending_migrations(pending_migrations: &migration::List) {
    println!("Pending migrations:");
    if pending_migrations.is_empty() {
        println!("{}", EM_DASH);
    } else {
        pending_migrations.iter().for_each(|migration| {
            println!("{}", migration.name());
        });
    }
}
