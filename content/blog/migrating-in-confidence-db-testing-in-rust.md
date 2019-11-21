+++
title = "DB testing in Rust"
date = 2019-02-19

[taxonomies]
tag = ["Rust", "postgres", "testing"]

[extra]
id = blog-single
+++

Writing integration tests for databases is often a cryptic art. The challenge stems from the fact that a database is stateful and instead we would like our tests to be reproducible. In this post I will share my experience writing database tests in Rust, some limitations of my current setup and thoughts on future improvements.
<!-- more -->

Note: As I continue to make progress on this, things might have changed and or improved :) Please have a peek at the code for the latest and greatest: https://github.com/toidiu/fin-public

### Motivation
The real benefit of strongly typed languages is that they give us the freedom to evolve our code at high mobility. This might seem trivial but has compounding effects over the lifetime of a codebase. However this guarantee breaks down at the edges, specifically at the database and routing edge for a webservice. So when faced with a Postgres schema migration, for a project I have been working on, *Fin*, I set out to write some integration tests and migrate with confidence.

The solution is heavily inspired from this (post)[https://medium.com/@ericdreichert/test-setup-and-teardown-in-rust-without-a-framework-ba32d97aa5ab].

### Goal
The task we have at hand is to create a Fixture that does the following:

- Access a database other than the one in production when running tests.
- Mirror our production database schema during testing.
- Populate fake data for testing.
- Clean up the data since we might make updates.
- Do the above per test.

### Application Setup
To help manage SQL schemas and migration for Fin, I use `diesel cli`. It is an awesome little tool that I manage along side the application. I can write `up.sql` and `down.sql` scripts per change and `run`, `revert` or `list` them.

For reference, the following is a susccient folder structure for *Fin*:
```
▾ fin/
  ▾ src/
      main.rs
    Cargo.lock
    Cargo.toml
▾ migrations/
  ▸ 00000000000000_diesel_initial_setup/
  ▾ 2018-10-07-022941_init/
      down.sql
      up.sql
  ▸ 2018-10-07-232226_fake_data/
```

### Solution

```rust

use chrono::prelude::*;
use postgres::{Connection, TlsMode};
use r2d2_postgres::{PostgresConnectionManager, TlsMode as R2TlsMode};
use std::env;
use std::fs;
use std::io::prelude::*;

const TEST_DB: &'static str = "fin_unit_test";
const CLUSTER_URI: &'static str = "postgres://postgres@localhost:5432";

lazy_static! {
    static ref DB_URI: String = format!("{}/{}", CLUSTER_URI, TEST_DB);
}

fn run_test<T>(test: T) -> ()
where
    T: FnOnce(&str) -> () + std::panic::UnwindSafe,
{
    let db_name = "make this unique per test"; // <- not currently used
    teardown();
    setup();
    let result = std::panic::catch_unwind(|| test(&db_name));
    assert!(result.is_ok())
}

setup() {
    // initialize a logger so we can see nice error messages during testing
    pretty_env_logger::try_init();

    // create the database
    let db_conn = Connection::connect(CLUSTER_URI, TlsMode::None)
        .expect("unable to create db conn");
    db_conn
        .execute(
            &format!(
                "CREATE DATABASE {name};",
                name = &TEST_DB.clone()
            ),
            &[],
        )
        .expect("unable to create db");

    // apply schema and add fake data
    let c_str = format!("{}/{}", CLUSTER_URI, TEST_DB);
    let conn = Connection::connect(DB_URI.as_str(), TlsMode::None).unwrap();
    let init =
        fs::read_to_string("../migrations/2018-10-07-022941_init/up.sql")
            .expect("file not found");
    let fake_data = fs::read_to_string(
        "../migrations/2018-10-07-232226_fake_data/up.sql",
    )
    .expect("file not found");

    conn.batch_execute(&init).unwrap();
    conn.batch_execute(&fake_data).unwrap();
}

fn teardown() {
    let db_conn = Connection::connect(CLUSTER_URI, TlsMode::None)
        .expect("unable to delete db conn");

    db_conn
        .execute(&format!("DROP database {};", &TEST_DB), &[])
        .expect("unable to delete db");
}
```

And then to use it in our tests:

```rust
#[test]
fn test_get_user() {
    run_test(|db_name| {
        let db = get_db();
        let res = db.get_user("apoorv@toidiu.com");
        assert_eq!(res.is_ok(), true);
        assert_eq!(res.unwrap().email, "apoorv@toidiu.com");
    })
}

```

### Limitations
- Wont work automatically on CI
- Need a separate test database
- Needs a script for inserting fake data
- Need to declare run_test closure per test (macro?)

### Closing thoughts
- Rust should natively support this functionality.
- This is a per test fixture, we still need a per suite fixture.
- Ideally we could start an in-memory postgres process for testing.
- Need a pre-test script to initialize stuff (start postgres process)



