mod queries;

use rusqlite::{params, Connection, Result, Row};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;
use std::io::stdout;
use std::ptr::null;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let conn = Connection::open("test.db")?;

    conn.execute(
        r#"
    CREATE TABLE IF NOT EXISTS users(
                      id INTEGER PRIMARY KEY AUTOINCREMENT ,
                      username TEXT NOT NULL UNIQUE,
                      email TEXT NOT NULL UNIQUE,
                      pass_hash BLOB,
                      salt BLOB,
                      created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
);"#,
        params!(),
    );

    let passhash: Vec<u8> = vec![0, 1];
    let salt: Vec<u8> = vec![2, 3];
    queries::reset(&conn)?;
    let new_id = queries::create_user(
        &conn,
        "testUser".parse().unwrap(),
        "test@testy.mctesterson.com".parse().unwrap(),
        passhash.clone(),
        salt.clone(),
    )?
    .unwrap()
    .id;
    let new_id = queries::create_user(
        &conn,
        "testUser2".parse().unwrap(),
        "test@testy.mctesterson1.com".parse().unwrap(),
        passhash.clone(),
        salt.clone(),
    )?
    .unwrap()
    .id;
    let new_id = queries::create_user(
        &conn,
        "testUser3".parse().unwrap(),
        "test@testy.mctesterson2.com".parse().unwrap(),
        passhash.clone(),
        salt.clone(),
    )?
    .unwrap()
    .id;
    let new_id = queries::create_user(
        &conn,
        "otherUser".parse().unwrap(),
        "test@testy.mctesterson3.com".parse().unwrap(),
        passhash.clone(),
        salt.clone(),
    )?
    .unwrap()
    .id;
    println!("{}", new_id);
    println!(
        "{}",
        serde_json::to_string_pretty(&queries::get_users(&conn)?).unwrap()
    );
    println!(
        "{}",
        serde_json::to_string_pretty(&queries::get_users_like(&conn, "%test%".parse().unwrap())?)
            .unwrap()
    );
    conn.execute(r#" VACUUM"#, params![])?;
    Ok(())
}
