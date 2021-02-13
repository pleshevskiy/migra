use crate::database::DatabaseConnection;
use crate::path::PathBuilder;
use crate::StdResult;
use std::fs;
use std::path::PathBuf;

pub trait Upgrade {
    fn upgrade(&self, connection: &mut DatabaseConnection) -> StdResult<()>;
}

pub trait Downgrade {
    fn downgrade(&self, connection: &mut DatabaseConnection) -> StdResult<()>;
}

#[derive(Debug)]
pub struct Migration {
    upgrade_sql: PathBuf,
    downgrade_sql: PathBuf,
    name: String,
}

impl Migration {
    pub(crate) fn new(directory: &PathBuf) -> Option<Migration> {
        if directory.is_dir() {
            let name = directory
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default();
            let upgrade_sql = PathBuilder::from(directory).append("up.sql").build();
            let downgrade_sql = PathBuilder::from(directory).append("down.sql").build();

            if upgrade_sql.exists() && downgrade_sql.exists() {
                return Some(Migration {
                    upgrade_sql,
                    downgrade_sql,
                    name: String::from(name),
                });
            }
        }

        None
    }
}

impl Migration {
    pub fn name(&self) -> &String {
        &self.name
    }
}

impl Upgrade for Migration {
    fn upgrade(&self, connection: &mut DatabaseConnection) -> StdResult<()> {
        let content = fs::read_to_string(&self.upgrade_sql)?;

        connection.create_migrations_table()?;
        connection.apply_sql(&content)?;
        connection.insert_migration_info(self.name())?;

        Ok(())
    }
}

impl Downgrade for Migration {
    fn downgrade(&self, connection: &mut DatabaseConnection) -> StdResult<()> {
        let content = fs::read_to_string(&self.downgrade_sql)?;

        connection.apply_sql(&content)?;
        connection.delete_migration_info(self.name())?;

        Ok(())
    }
}
