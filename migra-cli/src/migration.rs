use crate::database;
use crate::path::PathBuilder;
use postgres::Client;
use std::fs;
use std::path::PathBuf;

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

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn upgrade(&self, client: &mut Client) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let content = fs::read_to_string(&self.upgrade_sql)?;

        database::create_migrations_table(client)?;

        database::apply_sql(client, &content)?;

        database::insert_migration_info(client, self.name())?;

        Ok(())
    }

    pub fn downgrade(
        &self,
        client: &mut Client,
    ) -> Result<(), Box<dyn std::error::Error + 'static>> {
        let content = fs::read_to_string(&self.downgrade_sql)?;

        database::apply_sql(client, &content)?;

        database::delete_migration_info(client, self.name())?;

        Ok(())
    }
}
