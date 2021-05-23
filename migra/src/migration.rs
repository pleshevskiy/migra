use std::iter::FromIterator;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Migration {
    name: String,
}

impl Migration {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Migration {
            name: name.to_owned(),
        }
    }

    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct List {
    inner: Vec<Migration>,
}

impl<T: AsRef<str>> From<Vec<T>> for List {
    fn from(list: Vec<T>) -> Self {
        List {
            inner: list.iter().map(AsRef::as_ref).map(Migration::new).collect(),
        }
    }
}

impl std::ops::Deref for List {
    type Target = Vec<Migration>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl List {
    #[must_use]
    pub fn new() -> Self {
        List { inner: Vec::new() }
    }

    pub fn push(&mut self, migration: Migration) {
        self.inner.push(migration)
    }

    pub fn push_name(&mut self, name: &str) {
        self.inner.push(Migration::new(name))
    }

    #[must_use]
    pub fn contains(&self, other_migration: &Migration) -> bool {
        self.inner
            .iter()
            .any(|migration| migration == other_migration)
    }

    #[must_use]
    pub fn contains_name(&self, name: &str) -> bool {
        self.inner.iter().any(|migration| migration.name() == name)
    }

    #[must_use]
    pub fn maybe_next<'a>(&self, name: &'a str) -> Option<&'a str> {
        if self.contains_name(name) {
            None
        } else {
            Some(name)
        }
    }

    #[must_use]
    pub fn exclude(&self, list: &List) -> List {
        self.iter()
            .filter(|migration| !list.contains(migration))
            .collect()
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

        list.push(Migration::new(&String::from(SECOND_MIGRATION)));
        assert_eq!(list, List::from(vec![FIRST_MIGRATION, SECOND_MIGRATION]))
    }

    #[test]
    fn push_name_to_list() {
        let mut list = List::new();

        list.push_name(FIRST_MIGRATION);
        assert_eq!(list, List::from(vec![FIRST_MIGRATION]));

        list.push_name(&String::from(SECOND_MIGRATION));
        assert_eq!(list, List::from(vec![FIRST_MIGRATION, SECOND_MIGRATION]))
    }

    #[test]
    fn contains_migration() {
        let list = List::from(vec![FIRST_MIGRATION]);

        assert_eq!(list.contains(&Migration::new(FIRST_MIGRATION)), true);
        assert_eq!(list.contains(&Migration::new(SECOND_MIGRATION)), false);
    }

    #[test]
    fn contains_migration_name() {
        let list = List::from(vec![FIRST_MIGRATION]);

        assert_eq!(list.contains_name(FIRST_MIGRATION), true);
        assert_eq!(list.contains_name(SECOND_MIGRATION), false);
    }

    #[test]
    fn maybe_next_migration_name() {
        let list = List::from(vec![FIRST_MIGRATION]);

        assert_eq!(list.maybe_next(FIRST_MIGRATION), None);
        assert_eq!(list.maybe_next(SECOND_MIGRATION), Some(SECOND_MIGRATION));
    }

    #[test]
    fn create_excluded_migration_list() {
        let all_migrations = List::from(vec![FIRST_MIGRATION, SECOND_MIGRATION]);
        let applied_migrations = List::from(vec![FIRST_MIGRATION]);
        let excluded = all_migrations.exclude(&applied_migrations);

        assert_eq!(excluded, List::from(vec![SECOND_MIGRATION]))
    }
}