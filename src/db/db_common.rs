use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, Connection};

pub fn get_db_connection() -> Result<Pool<SqliteConnectionManager>, r2d2::Error> {
    let manager = SqliteConnectionManager::file("mmmbacktest.db");
    r2d2::Pool::new(manager)
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
