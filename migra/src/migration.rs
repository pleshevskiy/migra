#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Migration {
    name: String,
    upgrade_sql_content: Option<String>,
    downgrade_sql_content: Option<String>,
}

impl Migration {
    #[must_use]
    pub fn new(name: &str) -> Self {
        Migration {
            name: name.to_owned(),
            ..Migration::default()
        }
    }

    #[must_use]
    pub fn with_upgrade(name: &str, up_content: &str) -> Self {
        Migration {
            name: name.to_owned(),
            upgrade_sql_content: Some(up_content.to_owned()),
            ..Migration::default()
        }
    }

    #[must_use]
    pub fn with_upgrade_and_downgrade(name: &str, up_content: &str, down_content: &str) -> Self {
        Migration {
            name: name.to_owned(),
            upgrade_sql_content: Some(up_content.to_owned()),
            downgrade_sql_content: Some(down_content.to_owned()),
        }
    }

    #[must_use]
    pub fn name(&self) -> &String {
        &self.name
    }

    #[must_use]
    pub fn upgrade_sql_content(&self) -> Option<&String> {
        self.upgrade_sql_content.as_ref()
    }

    #[must_use]
    pub fn downgrade_sql_content(&self) -> Option<&String> {
        self.downgrade_sql_content.as_ref()
    }
}

#[derive(Debug, Clone, Default)]
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

impl List {
    #[must_use]
    pub fn new() -> Self {
        List { inner: Vec::new() }
    }

    pub fn push<T: AsRef<str>>(&mut self, name: &T) {
        self.inner.push(Migration::new(name.as_ref()))
    }

    #[must_use]
    pub fn contains(&self, name: &str) -> bool {
        self.inner.iter().any(|migration| migration.name() == name)
    }

    #[must_use]
    pub fn maybe_next<'a>(&self, name: &'a str) -> Option<&'a str> {
        if self.contains(name) {
            None
        } else {
            Some(name)
        }
    }
}
