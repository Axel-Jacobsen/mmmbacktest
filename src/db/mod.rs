mod bet_table;
pub mod db_common;
mod errors;
mod market_table;
mod user_table;

use rusqlite::{named_params, Connection, Result};
use serde_json::Value;

use crate::db::bet_table::rusqlite_row_to_bet;
use crate::db::errors::RowParsingError;
use crate::db::market_table::rusqlite_row_to_litemarket;
use crate::db::user_table::{rusqlite_row_to_user, DEFAULT_USER_ID};

/// Impls GET /v0/markets
/// Note that we filter the column 'creator_id' by
/// the query parameter 'user_id'.
/// Also note that groupId is not in the backtest data,
/// so it's ignored here.
/// Also, sort can't have the value last-comment-time because
/// there isn't a column for that in the backtest data.
fn get_markets(
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

pub fn get_market_by_slug(conn: &Connection, slug: &str) -> Result<Vec<Value>, RowParsingError> {
    let query = "SELECT * FROM markets WHERE url LIKE '%' || :slug || '%';";

    let mut stmt = conn.prepare(query)?;

    let market_iter = stmt.query_map(
        named_params! {
            ":slug": slug,
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

pub fn get_bets(
    conn: &Connection,
    user_id: Option<&str>,
    username: Option<&str>,
    contract_id: Option<&str>,
    contract_slug: Option<&str>,
    limit: Option<i64>,
    before: Option<&str>,
    after: Option<&str>,
    order: Option<&str>,
) -> Result<Vec<Value>, RowParsingError> {
    if let Some(contract_slug) = contract_slug {
        let markets = get_market_by_slug(conn, contract_slug)?;

        if markets.is_empty() {
            return Err(RowParsingError::MarketNotFound(
                format!("could not find market with slug {contract_slug}").to_string(),
            ));
        } else if markets.len() > 1 {
            return Err(RowParsingError::MarketNotFound(
                format!("found multiple markets with slug {contract_slug}").to_string(),
            ));
        } else {
            let market = markets.first().unwrap();
            let market_id_from_slug = Some(market.get("id").unwrap().as_str().unwrap());

            if contract_id.is_some() && contract_id != market_id_from_slug {
                return Err(RowParsingError::MarketNotFound(
                    "provided contract id does not match slug".to_string(),
                ));
            }
            return get_bets_by_params(
                conn,
                user_id,
                username,
                market_id_from_slug,
                None,
                limit,
                before,
                after,
                order,
            );
        }
    }

    let order = match order {
        Some("desc") => "DESC",
        _ => "ASC",
    };

    let query = format!(
        "SELECT * FROM bets
        WHERE
          (:user_id IS NULL OR user_id = :user_id) AND
          (:username IS NULL OR user_name = :username) AND
          (:contract_id IS NULL OR contract_id = :contract_id) AND
          (:before IS NULL OR created_time < (SELECT created_time FROM bets WHERE id = :before)) AND
          (:after IS NULL OR created_time > (SELECT created_time FROM bets WHERE id = :before))
        ORDER BY created_time {order}
        LIMIT :limit;"
    );

    let mut stmt = conn.prepare(&query)?;

    let bet_iter = stmt.query_map(
        named_params! {
            ":user_id": user_id,
            ":username": username,
            ":contract_id": contract_id,
            ":limit": limit.unwrap_or(500).min(1000),
            ":before": before,
            ":after": after,
        },
        |row| Ok(rusqlite_row_to_bet(row)),
    )?;

    let mut bets: Vec<Value> = Vec::new();
    for maybe_bet in bet_iter {
        // ??!! haha
        let bet = maybe_bet??;
        let bet_json = serde_json::to_value(bet)?;
        bets.push(bet_json);
    }

    Ok(bets)
}

pub fn get_bets_by_params(
    conn: &Connection,
    user_id: Option<&str>,
    username: Option<&str>,
    contract_id: Option<&str>,
    contract_slug: Option<&str>,
    limit: Option<i64>,
    before: Option<&str>,
    after: Option<&str>,
    order: Option<&str>,
) -> Result<Vec<Value>, RowParsingError> {
    get_bets(
        conn,
        user_id,
        username,
        contract_id,
        contract_slug,
        limit,
        before,
        after,
        order,
    )
}

pub fn get_me(conn: &Connection) -> Result<Value, RowParsingError> {
    let query = "SELECT * FROM users WHERE id = :default_user_id LIMIT 1;";

    let mut stmt = conn.prepare(query)?;

    let mut user_iter = stmt.query_map(
        named_params! {
            ":default_user_id": DEFAULT_USER_ID,
        },
        |row| Ok(rusqlite_row_to_user(row)),
    )?;

    let user = user_iter.next().expect("default user missing")??;
    let user_json = serde_json::to_value(user)?;

    Ok(user_json)
}
