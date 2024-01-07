mod bet_table;
mod db_common;
mod errors;
mod market_table;

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{named_params, Connection, Result};
use serde_json::Value;

use crate::db::bet_table::init_bet_table;
use crate::db::db_common::get_db_connection;
use crate::db::errors::RowParsingError;
use crate::db::market_table::{init_market_table, rusqlite_row_to_litemarket};

/// Impls GET /v0/markets
/// Note that we filter the column 'creator_id' by
/// the query parameter 'user_id'.
/// Also note that groupId is not in the backtest data,
/// so it's ignored here.
/// Also, sort can't have the value last-comment-time because
/// there isn't a column for that in the backtest data.
pub fn get_markets(
    conn: &Connection,
    id: Option<&str>,
    limit: Option<i64>,
    sort: Option<&str>,
    order: Option<&str>,
    before: Option<&str>,
    user_id: Option<&str>,
) -> Result<Vec<Value>, RowParsingError> {
    let order = match order {
        Some("desc") => "DESC",
        _ => "ASC",
    };

    let sort = match sort {
        Some("created-time") => "created-time",
        Some("updated-time") => "updated-time",
        Some("last-bet-time") => "last-bet-time",
        _ => "created-time",
    };

    let query = format!(
        "SELECT * FROM markets
        WHERE
          (:id is NULL OR id = :id) AND
          (:user_id IS NULL OR creator_id = :user_id) AND
          (:before IS NULL OR id < :before)
        ORDER BY
          CASE
            WHEN :sort = 'created-time' THEN created_time
            WHEN :sort = 'updated-time' THEN last_updated_time
            WHEN :sort = 'last-bet-time' THEN last_bet_time
          END {order}
        LIMIT :limit;"
    );

    let mut stmt = conn.prepare(&query)?;

    let market_iter = stmt.query_map(
        named_params! {
            ":id": id,
            ":limit": limit.unwrap_or(500).min(1000),
            ":sort": sort,
            ":before": before,
            ":user_id": user_id,
        },
        |row| Ok(rusqlite_row_to_litemarket(row)),
    )?;

    let mut markets: Vec<Value> = Vec::new();
    for maybe_market in market_iter {
        // ??!! haha
        let market = maybe_market??;
        let market_json = serde_json::to_value(market)?;
        markets.push(market_json);
    }

    Ok(markets)
}

pub fn get_markets_by_params(
    conn: &Connection,
    limit: Option<i64>,
    sort: Option<&str>,
    order: Option<&str>,
    before: Option<&str>,
    user_id: Option<&str>,
) -> Result<Vec<Value>, RowParsingError> {
    get_markets(conn, None, limit, sort, order, before, user_id)
}

pub fn get_markets_by_id(
    conn: &Connection,
    id: Option<&str>,
) -> Result<Vec<Value>, RowParsingError> {
    get_markets(conn, id, None, None, None, None, None)
}

pub fn setup_db() -> Pool<SqliteConnectionManager> {
    let connection_pool = get_db_connection().expect("failed to get db connection pool");
    let mut conn = connection_pool
        .get()
        .expect("failed to get db connection from the pool");

    init_market_table(&mut conn).expect("failed to init market table");
    init_bet_table(&mut conn).expect("failed to init bet table");

    connection_pool
}
