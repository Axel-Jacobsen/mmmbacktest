use log::debug;
use rusqlite::{params, Connection, Result};
use std::fs;

use crate::data_types::{FullMarket, LiteMarket};
use crate::db::db_common::*;

fn iter_over_markets(market_json: &String) -> Vec<FullMarket> {
    let file_as_string = fs::read_to_string(market_json).unwrap();
    let markets: Vec<FullMarket> = serde_json::from_str(&file_as_string).unwrap();
    markets
}

pub fn create_market_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE markets (
            id TEXT PRIMARY KEY,
            creator_id TEXT NOT NULL,
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

pub fn bulk_insert_markets(conn: &mut Connection, markets: &Vec<LiteMarket>) -> Result<usize> {
    let stmt_str = "INSERT INTO markets (
        id, creator_id, creator_username, creator_name, creator_avatar_url, close_time,
        created_time, question, url, outcome_type, mechanism, probability,
        pool, p, total_liquidity, value, min, max, is_log_scale, volume,
        volume_24_hours, is_resolved, resolution_time, resolution,
        resolution_probability, last_updated_time, last_bet_time
    ) VALUES (
        ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11,
        ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19,
        ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27
    );";

    for chunk in markets.chunks(1000) {
        let tx = conn.transaction()?;

        {
            // scope so tx borrow (from prepare) is OK
            let mut stmt = tx.prepare(stmt_str)?;
            for market in chunk {
                stmt.execute(params![
                    market.id,
                    market.creator_id,
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

    Ok(markets.len())
}

pub fn init_market_table(conn: &mut Connection) -> Result<usize> {
    if !table_exists(conn, "markets")? {
        debug!("creating 'markets' table");
        create_market_table(conn)?;
    } else {
        debug!("found 'bets' table");
    }

    let mut count = 0;

    // TODO really we should check that the number of rows equals the number of bets,
    // or maybe just check if all the ids are in the db and insert the missing ones?
    let num_rows = count_rows(conn, "markets").expect("failed to count rows in markets table");
    if count_rows(conn, "markets").expect("failed to count rows in markets table") == 0 {
        debug!("inserting markets...");

        let markets =
            iter_over_markets(&"backtest-data/manifold-dump-markets-04082023.json".to_string());

        // Pull the lite markets out, because I don't want to deal w/ full market for now. Changing this
        // will be easy once we actually need the full markets!
        let lite_markets: Vec<LiteMarket> =
            markets.iter().map(|fm| fm.lite_market.clone()).collect();

        count = bulk_insert_markets(conn, &lite_markets)?;

        debug!("{count} markets inserted...");
    } else {
        // TODO
        debug!("there are {num_rows} (instead of 0) rows, so not inserting anything");
    }

    Ok(count)
}
