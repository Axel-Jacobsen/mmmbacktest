use rusqlite::{params, Connection, Result};

pub fn get_db_connection() -> Result<Connection> {
    Connection::open("mmmbacktest.db")
}

pub fn table_exists(conn: &Connection, table_name: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name=?1")?;
    let exists = stmt.exists(params![table_name])?;
    Ok(exists)
}

pub fn count_rows(conn: &Connection, table_name: &str) -> Result<usize> {
    let mut stmt = conn.prepare(&format!("SELECT COUNT(*) FROM {}", table_name))?;
    let count: usize = stmt.query_row([], |row| row.get(0))?;
    Ok(count)
}
