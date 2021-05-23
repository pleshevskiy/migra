#![deny(missing_debug_implementations)]
#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::missing_errors_doc)]

mod error;
pub mod fs;
pub mod migration;

pub use error::{Error, MigraResult as Result};

/*

# list

fs::get_all_migrations()
db::get_applied_migrations()
utils::filter_pending_migrations(all_migrations, applied_migrations)
show_migrations(applied_migrations)
show_migrations(pending_migrations)


# upgrade

fs::get_all_migrations()
db::get_applied_migrations()
utils::filter_pending_migrations(all_migrations, applied_migrations)

db::upgrade_migration()



# downgrade




*/
