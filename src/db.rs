use diesel::{prelude::QueryResult, sqlite::SqliteConnection, Connection, RunQueryDsl};


use serde_json::Value;

use std::collections::HashMap;
use std::fs;

use crate::data_types::{Bet, LiteMarket, FullMarket};
use crate::schema::bets::dsl::*;

// https://chat.openai.com/share/d2e2e7a3-80ed-4502-b090-9eb6237c5c74
// Geez this is harder than I thought. I hope that throwing it all into a sqlite db
// won't be too painful. I wish I knew more about this. Enough for today. Why is sql
// serialization so ugly?? I hate that I have to go convert json to a rust struct to
// convert it to a sqlite struct, just to be converted back to a rust struct and then
// serialized back into json. That's awful.
//
// Actually, there *has* to be a better way. This is insane. Maybe just making my own index is
// fine.

pub fn establish_connection() -> SqliteConnection {
    SqliteConnection::establish("mmmbacktest.sqlite3")
        .unwrap_or_else(|_| panic!("Error connecting to database"))
}

fn check_table_exists(conn: &mut SqliteConnection, table_name: &str) -> bool {
    let query = format!("SELECT 1 FROM {} LIMIT 1", table_name);
    diesel::sql_query(query).execute(conn).is_ok()
}


fn create_tables_if_not_exists(conn: &mut SqliteConnection) -> QueryResult<usize> {
    diesel::sql_query(
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
        )"
    ).execute(conn)
}


pub fn setup_db() -> SqliteConnection {
    let mut conn = establish_connection();
    if !check_table_exists(&mut conn, "bets") {
        create_tables_if_not_exists(&mut conn);
    }
    conn
}

fn insert_bets(conn: &SqliteConnection, betss: Vec<Bet>) {
    diesel::insert_into(bets::table)
        .values(&betss)
        .execute(conn)
        .expect("Error inserting Bet")
}
