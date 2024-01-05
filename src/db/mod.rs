pub mod bet_table;
pub mod market_table;

use log::debug;
use rusqlite::{params, Connection, Result};

use std::fs;

use crate::data_types::{Bet, FullMarket, LiteMarket};

use crate::db::bet_table::{bulk_insert_bets, create_bet_table};
use crate::db::market_table::{bulk_insert_markets, create_market_table};

fn iter_over_markets(market_json: &String) -> Vec<FullMarket> {
    let file_as_string = fs::read_to_string(market_json).unwrap();
    let markets: Vec<FullMarket> = serde_json::from_str(&file_as_string).unwrap();
    markets
}

fn iter_over_bets(bets_dir: &str) -> impl Iterator<Item = Vec<Bet>> {
    let paths = fs::read_dir(bets_dir).unwrap_or_else(|err| {
        panic!("Error reading directory: {}", err);
    });

    paths.filter_map(|entry| {
        entry.ok().and_then(|e| {
            let path = e.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                let file_as_string =
                    fs::read_to_string(path).expect("couldn't read file to string");

                serde_json::from_str(&file_as_string).expect("failed to parse json into Bet struct")
            } else {
                None
            }
        })
    })
}

fn get_db_connection() -> Result<Connection> {
    Connection::open("mmmbacktest.db")
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

fn init_market_table(conn: &mut Connection) -> Result<usize> {
    if !table_exists(conn, "markets")? {
        debug!("creating 'markets' table");
        create_market_table(conn)?;
    }

    let mut count = 0;

    if count_rows(conn, "markets").expect("failed to count rows in markets table") == 0 {
        debug!("inserting markets...");

        let markets =
            iter_over_markets(&"backtest-data/manifold-dump-markets-04082023.json".to_string());

        let lite_markets: Vec<LiteMarket> =
            markets.iter().map(|fm| fm.lite_market.clone()).collect();

        count = bulk_insert_markets(conn, &lite_markets)?;

        debug!("{count} markets inserted...");
    }

    Ok(count)
}

fn init_bet_table(conn: &mut Connection) -> Result<usize> {
    if !table_exists(conn, "bets")? {
        debug!("creating 'bets' table");
        create_bet_table(conn)?;
    }

    let mut count = 0;

    if count_rows(conn, "bets").expect("failed to count rows in bets table") == 0 {
        debug!("inserting bets");

        for bets in iter_over_bets("backtest-data/bets") {
            count += bulk_insert_bets(conn, &bets)?;
        }

        debug!("{count} bets inserted...");
    }

    Ok(count)
}

pub fn setup_db() -> Connection {
    let mut conn = get_db_connection().expect("failed to get db connection");

    init_market_table(&mut conn).expect("failed to init market table");
    init_bet_table(&mut conn).expect("failed to init bet table");

    conn
}
