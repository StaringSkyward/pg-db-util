use postgres::{Client, NoTls};
use std::fs;

mod embedded {
  use refinery::embed_migrations;
  embed_migrations!("migrations");
}

pub struct ConnectionProperties<'cp> {
  pub db_host: &'cp str,
  pub db_name: &'cp str,
  pub db_username: &'cp str,
  pub db_password: &'cp str,
}

fn connectionstring_with_db(connection_props: &ConnectionProperties) -> String {
  let mut connstring = connectionstring(&connection_props);

  if !connection_props.db_name.trim().is_empty() {
    connstring = format!("{} dbname={}", connstring, connection_props.db_name);
  }

  connstring
}

fn connectionstring(connection_props: &ConnectionProperties) -> String {
  let mut connstring = String::new();

  if !connection_props.db_host.trim().is_empty() {
    connstring = format!("host={}", connection_props.db_host);
  }

  if !connection_props.db_username.trim().is_empty() {
    connstring = format!("{} user={}", connstring, connection_props.db_username);
  }

  if !connection_props.db_password.trim().is_empty() {
    connstring = format!("{} password={}", connstring, connection_props.db_password);
  }

  connstring
}

pub fn drop(connection_props: &ConnectionProperties) -> Result<u64, tokio_postgres::error::Error> {
  let connstring = connectionstring(&connection_props);
  let mut client = Client::connect(&connstring, NoTls)?;
  let query = format!("DROP DATABASE IF EXISTS {};", &connection_props.db_name);

  client.execute(query.as_str(), &[])
}

pub fn create(
  connection_props: &ConnectionProperties,
) -> Result<u64, tokio_postgres::error::Error> {
  let connstring = connectionstring(&connection_props);
  let mut client = Client::connect(&connstring, NoTls)?;
  let query = format!(
    "CREATE DATABASE {} WITH OWNER={} LC_COLLATE='en_GB.utf8' LC_CTYPE='en_GB.utf8' TEMPLATE=template0 ENCODING=UTF8;",
    &connection_props.db_name, &connection_props.db_username
  );

  client.execute(query.as_str(), &[])
}

pub fn migrate(connection_props: &ConnectionProperties) {
  let connstring = connectionstring_with_db(&connection_props);
  let mut client = Client::connect(&connstring, NoTls).unwrap();
  embedded::migrations::runner().run(&mut client).unwrap();
}

pub fn seed(connection_props: &ConnectionProperties, seed_file: &str) {
  let seed_query = fs::read_to_string(seed_file).expect("Unable to read seed file");
  let connstring = connectionstring_with_db(&connection_props);
  let mut client = Client::connect(&connstring, NoTls).unwrap();
  client.simple_query(&seed_query).unwrap();
}
