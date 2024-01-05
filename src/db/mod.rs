mod bet_table;
mod db_common;
mod market_table;

use rusqlite::Connection;

use crate::db::bet_table::init_bet_table;
use crate::db::db_common::get_db_connection;
use crate::db::market_table::init_market_table;



pub fn setup_db() -> Connection {
    let mut conn = get_db_connection().expect("failed to get db connection");

    init_market_table(&mut conn).expect("failed to init market table");
    init_bet_table(&mut conn).expect("failed to init bet table");

    conn
}
