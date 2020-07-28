# The Ickle PG DB Util

A simple utility written in [Rust](https://www.rust-lang.org) to allow easy creation, migration, seeding and dropping of a PostgreSQL database. It is most suited for use in containers.

The code quality probably leaves a lot to be desired as the author is very much a Rust beginner - any help with improvements gratefully accepted.

## Building

You'll need Rust installed on the machine that you intend to do this on.

1. Clone this repo wherever you need it
2. Add migration files in the `migrations` folder -see the [Refinery Examples](https://github.com/rust-db/refinery) for detailed info on how embedded migrations work and [Migrations](##migrations)
3. Build the app: `cargo build --release`

## Running

Once you have built the app you can run it directly from `./target/release/db` or copy it to wherever you need it. Run it without any environment variables or options set to see help.

Note that to seed your database you'll need to pass the path a seed file which contains the plain SQL statments.

## Migrations

```sql
--- Save this file as e.g. migrations/V1__initial.sql or migrations/V1__create_mytable.sql
CREATE EXTENSION IF NOT EXISTS pgcrypto;

CREATE TABLE mytable (
  id UUID PRIMARY KEY NOT NULL DEFAULT GEN_RANDOM_UUID(),
  name VARCHAR(255) NOT NULL,
  created_at DATE NOT NULL DEFAULT CURRENT_DATE,
  updated_at DATE NOT NULL DEFAULT CURRENT_DATE
);

CREATE INDEX IF NOT EXISTS mytable_name ON mytable (name);
```
