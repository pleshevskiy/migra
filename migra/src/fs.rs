use crate::errors::MigraResult;
use crate::migration;
use std::io;
use std::path::Path;

/// Checks if the directory is a migration according to the principles of the crate.
#[must_use]
pub fn is_migration_dir(path: &Path) -> bool {
    path.join("up.sql").exists() && path.join("down.sql").exists()
}

/// Get all migration directories from path and returns as [List].
///
/// This utility checks if the directory is a migration. See [`is_migration_dir`] for
/// more information.
///
/// [List]: migration::List
/// [is_migration_dir]: fs::is_migration_dir
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
