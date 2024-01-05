mod bet_table;
mod db_common;
mod market_table;

use rusqlite::{named_params, Connection, Result};

use crate::db::bet_table::init_bet_table;
use crate::db::db_common::get_db_connection;
use crate::db::market_table::init_market_table;

pub fn get_markets(
    conn: &Connection,
    limit: Option<i64>,
    sort: Option<&str>,
    order: Option<&str>,
    before: Option<&str>,
    user_id: Option<&str>,
    group_id: Option<&str>,
) -> Result<Vec<serde_json::Value>> {
    let mut query = String::from("SELECT * FROM Markets WHERE ");
    let mut conditions = Vec::new();
    let mut params = Vec::new();

    if let Some(before) = before {
        conditions.push("id < :before");
        params.push(("before", before));
    }
    if let Some(user_id) = user_id {
        conditions.push("creator_id = :user_id");
        params.push(("user_id", user_id));
    }
    if let Some(group_id) = group_id {
        conditions.push("group_id = :group_id");
        params.push(("group_id", group_id));
    }

    query += &conditions.join(" AND ");

    let sort_column = match sort.unwrap_or("created-time") {
        "updated-time" => "updated_time",
        "last-bet-time" => "last_bet_time",
        "last-comment-time" => "last_comment_time",
        _ => "created_time",
    };

    let order = if order == Some("asc") { "ASC" } else { "DESC" };
    query += &format!(" ORDER BY {} {}", sort_column, order);

    let limit = limit.unwrap_or(500).min(1000); // Ensure limit is not more than 1000
    query += &format!(" LIMIT {}", limit);

    let mut stmt = conn.prepare(&query)?;

    let rows = stmt.query_map(
        named_params! {
            ":before": before,
            ":user_id": user_id,
            ":group_id": group_id,
        },
        |row| {
            println!("row: {:?}", row);
            Ok(())
        },
    )?;

    Ok(vec![])
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
