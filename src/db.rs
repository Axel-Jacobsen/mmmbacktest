use rusqlite::{params, Connection, Result};

use serde_json::Value;

use std::collections::HashMap;
use std::fs;

use crate::data_types::{Bet, FullMarket, LiteMarket};

// https://chat.openai.com/share/d2e2e7a3-80ed-4502-b090-9eb6237c5c74
// Geez this is harder than I thought. I hope that throwing it all into a sqlite db
// won't be too painful. I wish I knew more about this. Enough for today. Why is sql
// serialization so ugly?? I hate that I have to go convert json to a rust struct to
// convert it to a sqlite struct, just to be converted back to a rust struct and then
// serialized back into json. That's awful.
//
// Actually, there *has* to be a better way. This is insane. Maybe just making my own index is
// fine.

fn get_db_connection() -> Result<Connection> {
    Connection::open("my_database.db")
}

fn create_db_tables(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE bets (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            user_avatar_url TEXT,
            user_name TEXT,
            user_username TEXT,
            contract_id TEXT NOT NULL,
            answer_id TEXT,
            created_time BIGINT NOT NULL,
            amount FLOAT NOT NULL,
            loan_amount FLOAT,
            outcome TEXT NOT NULL,
            shares FLOAT NOT NULL,
            prob_before FLOAT NOT NULL,
            prob_after FLOAT NOT NULL,
            is_api BOOL,
            is_ante BOOL NOT NULL,
            is_redemption BOOL NOT NULL,
            is_challenge BOOL NOT NULL,
            challenge_slug TEXT,
            reply_to_comment_id TEXT,
            fees TEXT,
            limit_props TEXT,
            shares_by_outcome TEXT
        )",
        [],
    )?;
    Ok(())
}

fn table_exists(conn: &Connection, table_name: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?1")?;
    let exists = stmt.exists(params![table_name])?;
    Ok(exists)
}

fn count_rows(conn: &Connection, table_name: &str) -> Result<usize> {
    let mut stmt = conn.prepare(&format!("SELECT COUNT(*) FROM {}", table_name))?;
    let count: usize = stmt.query_row([], |row| row.get(0))?;
    Ok(count)
}

pub fn setup_db() -> Connection {
    let conn = get_db_connection().expect("failed to get db connection");
    if !table_exists(&conn, "bets").expect("failed to check if table exists") {
        create_db_tables(&conn).expect("failed to create db tables - perhaps they already exist?");
    }

    conn
}
