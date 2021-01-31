use postgres::{Client, NoTls, Error};

pub fn connect(connection_string: &str) -> Result<Client, Error> {
    Client::connect(connection_string, NoTls)
}

pub fn apply_sql(client: &mut Client, sql_content: &str) -> Result<(), Error> {
    client.batch_execute(sql_content)
}
