use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection};
use std::sync::Arc;

use crate::db::bet_table::init_bet_table;
use crate::db::market_table::init_market_table;

pub fn get_db_connection_pool() -> Result<Arc<Pool<SqliteConnectionManager>>, r2d2::Error> {
    let manager = SqliteConnectionManager::file("mmmbacktest.db");
    Ok(Arc::new(r2d2::Pool::new(manager)?))
}

pub fn setup_db() -> Arc<Pool<SqliteConnectionManager>> {
    let connection_pool = get_db_connection_pool().expect("failed to get db connection pool");
    let mut conn = get_db_connection(connection_pool.clone());

    init_market_table(&mut conn).expect("failed to init market table");
    init_bet_table(&mut conn).expect("failed to init bet table");

    connection_pool
}

pub fn get_db_connection(
    connection_pool: Arc<Pool<SqliteConnectionManager>>,
) -> PooledConnection<SqliteConnectionManager> {
    let conn = connection_pool
        .get()
        .expect("failed to get db connection from the pool");

    // https://phiresky.github.io/blog/2020/sqlite-performance-tuning/
    let query = "
        pragma journal_mode = WAL;
        pragma synchronous = normal;
        pragma temp_store = memory;
        pragma mmap_size = 30000000000;";

    {
        match conn.execute(query, []) {
            Ok(_) => {}
            Err(e) => {
                log::error!("failed to optimize db connection: {e}");
            }
        };
    }

    conn
}

pub fn table_exists(conn: &Connection, table_name: &str) -> rusqlite::Result<bool> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?1")?;
    let exists = stmt.exists(params![table_name])?;
    Ok(exists)
}

pub fn count_rows(conn: &Connection, table_name: &str) -> rusqlite::Result<usize> {
    let mut stmt = conn.prepare(&format!("SELECT COUNT(*) FROM {}", table_name))?;
    let count: usize = stmt.query_row([], |row| row.get(0))?;
    Ok(count)
}
