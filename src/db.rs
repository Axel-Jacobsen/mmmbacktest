use rusqlite::{params, Connection, Result};

use serde_json::Value;

use std::{fs, collections::HashMap};

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
    Connection::open("mmmbacktest.db")
}

fn create_bet_table(conn: &Connection) -> Result<()> {
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

fn create_litemarket_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE LiteMarket (
            id TEXT PRIMARY KEY,
            creator_username TEXT NOT NULL,
            creator_name TEXT NOT NULL,
            creator_avatar_url TEXT,
            close_time INTEGER,
            created_time INTEGER NOT NULL,
            question TEXT NOT NULL,
            url TEXT NOT NULL,
            outcome_type TEXT NOT NULL,
            mechanism TEXT NOT NULL,
            probability REAL,
            pool TEXT,
            p REAL,
            total_liquidity REAL,
            value REAL,
            min REAL,
            max REAL,
            is_log_scale INTEGER,
            volume REAL NOT NULL,
            volume_24_hours REAL NOT NULL,
            is_resolved INTEGER NOT NULL,
            resolution_time INTEGER,
            resolution TEXT,
            resolution_probability REAL,
            last_updated_time INTEGER,
            last_bet_time INTEGER
        )",
        [],
    )?;
    Ok(())
}

pub fn bulk_insert_markets(conn: &mut Connection, markets: &Vec<LiteMarket>) -> Result<()> {
    let stmt_str = "INSERT INTO LiteMarket (
        id, creator_username, creator_name, creator_avatar_url, close_time,
        created_time, question, url, outcome_type, mechanism, probability,
        pool, p, total_liquidity, value, min, max, is_log_scale, volume,
        volume_24_hours, is_resolved, resolution_time, resolution,
        resolution_probability, last_updated_time, last_bet_time
    ) VALUES (
        ?1, ?2, ?3, ?4, ?5,
        ?6, ?7, ?8, ?9, ?10, ?11,
        ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19,
        ?20, ?21, ?22, ?23,
        ?24, ?25, ?26
    );";

    for chunk in markets.chunks(1000) {
        let tx = conn.transaction()?;

        {
            // scope so tx borrow (from prepare) is OK
            let mut stmt = tx.prepare(stmt_str)?;
            for market in chunk {
                stmt.execute(params![
                    market.id,
                    market.creator_username,
                    market.creator_name,
                    market.creator_avatar_url,
                    market.close_time,
                    market.created_time,
                    market.question,
                    market.url,
                    serde_json::to_string(&market.outcome_type).unwrap(),
                    serde_json::to_string(&market.mechanism).unwrap(),
                    market.probability,
                    serde_json::to_string(&market.pool).unwrap(),
                    market.p,
                    market.total_liquidity,
                    market.value,
                    market.min,
                    market.max,
                    market.is_log_scale,
                    market.volume,
                    market.volume_24_hours,
                    market.is_resolved,
                    market.resolution_time,
                    market.resolution,
                    market.resolution_probability,
                    market.last_updated_time,
                    market.last_bet_time
                ])?;
            }
        }

        tx.commit()?;
    }

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
        create_bet_table(&conn).expect("failed to create db tables - perhaps they already exist?");
        // go through bets and insert them
    }

    if !table_exists(&conn, "markets").expect("failed to check if table exists") {
        create_litemarket_table(&conn).expect("failed to create db tables - perhaps they already exist?");
        // go through markets and insert them
    }

    conn
}
