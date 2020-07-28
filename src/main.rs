use clap::{App, AppSettings, Arg};
use std::env;
use std::process;

mod db;

fn missing_env_vars() {
  println!(
    "ERROR: missing environment variables.
Ensure all of the following environment variables are set:
DB_HOST DB_NAME DB_USER DB_PASSWORD"
  );
  process::exit(1);
}

fn main() {
  // Set connection creds from env vars
  let mut db_host = String::new();
  let mut db_name = String::new();
  let mut db_user = String::new();
  let mut db_password = String::new();

  match env::var("DB_HOST") {
    Ok(val) => {
      if val.trim().is_empty() {
        println!("ERROR: DB_HOST is empty!");
        process::exit(1);
      } else {
        db_host = val
      }
    }
    Err(_e) => {
      missing_env_vars();
    }
  }

  match env::var("DB_NAME") {
    Ok(val) => {
      if val.trim().is_empty() {
        println!("ERROR: DB_NAME is empty!");
        process::exit(1);
      } else {
        db_name = val
      }
    }
    Err(_e) => {
      missing_env_vars();
    }
  }

  match env::var("DB_USER") {
    Ok(val) => {
      if val.trim().is_empty() {
        println!("ERROR: DB_USER is empty!");
        process::exit(1);
      } else {
        db_user = val
      }
    }
    Err(_e) => {
      missing_env_vars();
    }
  }

  match env::var("DB_PASSWORD") {
    Ok(val) => {
      if val.trim().is_empty() {
        println!("ERROR: DB_PASSWORD is empty!");
        process::exit(1);
      } else {
        db_password = val
      }
    }
    Err(_e) => {
      missing_env_vars();
    }
  }

  let conn_properties = db::ConnectionProperties {
    db_host: &db_host,
    db_name: &db_name,
    db_username: &db_user,
    db_password: &db_password,
  };

  // Parse command line args or show help
  let matches = App::new("PostgreSQL DB Helper")
    .version("0.1.0")
    .author("Matt <staringskyward@gmail.com>")
    .about("A utility for creating, migrating, seeding and dropping your postgres DB")
    .arg(
      Arg::with_name("create")
        .short("c")
        .long("create")
        .takes_value(false)
        .conflicts_with("drop")
        .help("Create the database"),
    )
    .arg(
      Arg::with_name("drop")
        .short("d")
        .long("drop")
        .takes_value(false)
        .conflicts_with("create")
        .help("Drop the database"),
    )
    .arg(
      Arg::with_name("migrate")
        .short("m")
        .long("migrate")
        .takes_value(false)
        .conflicts_with("drop")
        .help("Migrate the database"),
    )
    .arg(
      Arg::with_name("seed")
        .short("s")
        .long("seed")
        .takes_value(true)
        .required(true)
        .conflicts_with("drop")
        .help("Insert seed data from the specified SQL file"),
    )
    .setting(AppSettings::ArgRequiredElseHelp)
    .get_matches();

  if matches.is_present("drop") {
    match db::drop(&conn_properties) {
      Ok(_val) => println!("Database {} dropped.", conn_properties.db_name),
      Err(e) => {
        eprintln!("{}", e);
        process::exit(1);
      }
    }
  }

  if matches.is_present("create") {
    match db::create(&conn_properties) {
      Ok(_val) => println!("Database {} created.", conn_properties.db_name),
      Err(e) => {
        eprintln!("{}", e);
        process::exit(1);
      }
    }
  }

  if matches.is_present("migrate") {
    db::migrate(&conn_properties);
    println!("Database {} migrated.", conn_properties.db_name);
  }

  if matches.is_present("seed") {
    if let Some(ref seed_file) = matches.value_of("seed") {
      db::seed(&conn_properties, seed_file);
      println!("Database {} seeded.", conn_properties.db_name);
    } else {
      println!("No seed file passed")
    }
  }
}
