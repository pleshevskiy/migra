use crate::errors::MigraResult;
use crate::migration;
use crate::migration::{DOWNGRADE_MIGRATION_FILE_NAME, UPGRADE_MIGRATION_FILE_NAME};
use std::io;
use std::path::Path;

#[must_use]
pub fn is_migration_dir(path: &Path) -> bool {
    path.join(UPGRADE_MIGRATION_FILE_NAME).exists()
        && path.join(DOWNGRADE_MIGRATION_FILE_NAME).exists()
}

pub fn get_all_migrations(dir_path: &Path) -> MigraResult<migration::List> {
    let mut entries = match dir_path.read_dir() {
        Err(e) if e.kind() == io::ErrorKind::NotFound => vec![],
        entries => entries?
            .filter_map(|res| res.ok().map(|e| e.path()))
            .filter(|path| is_migration_dir(&path))
            .collect::<Vec<_>>(),
    };

    if entries.is_empty() {
        return Ok(migration::List::new());
    }

    entries.sort();
    Ok(migration::List::from(entries))
}

#[must_use]
pub fn filter_pending_migrations(
    all_migrations: &migration::List,
    applied_migrations: &migration::List,
) -> migration::List {
    all_migrations.exclude(applied_migrations)
}
