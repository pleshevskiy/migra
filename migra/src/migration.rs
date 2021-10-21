use crate::errors::MigraResult;
use crate::managers::ManageMigrations;
use std::iter::FromIterator;

/// A simple wrap over string.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Migration {
    name: String,
}

impl Migration {
    /// Creates new migration by name.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Migration {
            name: name.to_owned(),
        }
    }

    /// Returns name of migration.
    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }
}

/// Wrap over migration vector. Can be implicitly converted to a vector and has
/// a few of additional utilities for handling migrations.
///
/// Can be presented as a list of all migrations, a list of pending migrations
/// or a list of applied migrations, depending on the implementation.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct List {
    inner: Vec<Migration>,
}

impl<T: AsRef<std::path::Path>> From<Vec<T>> for List {
    fn from(list: Vec<T>) -> Self {
        List {
            inner: list
                .iter()
                .map(AsRef::as_ref)
                .map(|path| {
                    path.file_name()
                        .and_then(std::ffi::OsStr::to_str)
                        .expect("Cannot read migration name")
                })
                .map(Migration::new)
                .collect(),
        }
    }
}

impl From<Vec<Migration>> for List {
    fn from(list: Vec<Migration>) -> Self {
        List { inner: list }
    }
}

impl FromIterator<Migration> for List {
    fn from_iter<I: IntoIterator<Item = Migration>>(iter: I) -> Self {
        let mut list = List::new();

        for item in iter {
            list.push(item);
        }

        list
    }
}

impl<'a> FromIterator<&'a Migration> for List {
    fn from_iter<I: IntoIterator<Item = &'a Migration>>(iter: I) -> Self {
        let mut list = List::new();

        for item in iter {
            list.push(item.clone());
        }

        list
    }
}

impl std::ops::Deref for List {
    type Target = Vec<Migration>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl List {
    /// Creates empty migration list.
    #[must_use]
    pub fn new() -> Self {
        List { inner: Vec::new() }
    }

    /// Push migration to list.
    pub fn push(&mut self, migration: Migration) {
        self.inner.push(migration);
    }

    /// Push migration name to list.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use migra::migration::List;
    /// # let mut list = List::new();
    /// list.push_name("name");
    /// # assert_eq!(list, List::from(vec!["name"]));
    /// ```
    ///
    /// Is identical to the following
    /// ```rust
    /// # use migra::migration::{List, Migration};
    /// # let mut list = List::new();
    /// list.push(Migration::new("name"));
    /// # assert_eq!(list, List::from(vec!["name"]));
    /// ```
    pub fn push_name(&mut self, name: &str) {
        self.inner.push(Migration::new(name));
    }

    /// Check if list contains specific migration.
    #[must_use]
    pub fn contains(&self, other_migration: &Migration) -> bool {
        self.inner
            .iter()
            .any(|migration| migration == other_migration)
    }

    /// Check if list contains migration with specific name.
    #[must_use]
    pub fn contains_name(&self, name: &str) -> bool {
        self.inner.iter().any(|migration| migration.name() == name)
    }

    /// Exclude specific list from current list.
    #[must_use]
    pub fn exclude(&self, list: &List) -> List {
        self.inner
            .iter()
            .filter(|migration| !list.contains_name(migration.name()))
            .collect()
    }

    /// Runs a upgrade migration with SQL content and adds a new migration to the current list
    /// If there is no migration migration with specific name in the list.
    pub fn should_run_upgrade_migration(
        &mut self,
        client: &mut dyn ManageMigrations,
        name: &str,
        content: &str,
    ) -> MigraResult<bool> {
        let is_missed = !self.contains_name(name);

        if is_missed {
            client.run_upgrade_migration(name, content)?;
            self.push_name(name);
        }

        Ok(is_missed)
    }

    /// Runs a downgrade migration with SQL content and removes the last migration from the
    /// current list if the last item in the list has the specified name.
    pub fn should_run_downgrade_migration(
        &mut self,
        client: &mut dyn ManageMigrations,
        name: &str,
        content: &str,
    ) -> MigraResult<bool> {
        let is_latest = self.inner.last() == Some(&Migration::new(name));

        if is_latest {
            client.run_downgrade_migration(name, content)?;
            self.inner.pop();
        }

        Ok(is_latest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const FIRST_MIGRATION: &str = "initial_migration";
    const SECOND_MIGRATION: &str = "new_migration";

    #[test]
    fn push_migration_to_list() {
        let mut list = List::new();

        list.push(Migration::new(FIRST_MIGRATION));
        assert_eq!(list, List::from(vec![FIRST_MIGRATION]));

        list.push(Migration::new(SECOND_MIGRATION));
        assert_eq!(list, List::from(vec![FIRST_MIGRATION, SECOND_MIGRATION]));
    }

    #[test]
    fn push_name_to_list() {
        let mut list = List::new();

        list.push_name(FIRST_MIGRATION);
        assert_eq!(list, List::from(vec![FIRST_MIGRATION]));

        list.push_name(&String::from(SECOND_MIGRATION));
        assert_eq!(list, List::from(vec![FIRST_MIGRATION, SECOND_MIGRATION]));
    }

    #[test]
    fn contains_migration() {
        let list = List::from(vec![FIRST_MIGRATION]);

        assert!(list.contains(&Migration::new(FIRST_MIGRATION)));
        assert!(!list.contains(&Migration::new(SECOND_MIGRATION)));
    }

    #[test]
    fn contains_migration_name() {
        let list = List::from(vec![FIRST_MIGRATION]);

        assert!(list.contains_name(FIRST_MIGRATION));
        assert!(!list.contains_name(SECOND_MIGRATION));
    }

    #[test]
    fn create_excluded_migration_list() {
        let all_migrations = List::from(vec![FIRST_MIGRATION, SECOND_MIGRATION]);
        let applied_migrations = List::from(vec![FIRST_MIGRATION]);
        let excluded = all_migrations.exclude(&applied_migrations);

        assert_eq!(excluded, List::from(vec![SECOND_MIGRATION]));
    }
}
