use log::debug;
use rusqlite::{params, Connection, Result, Row};
use std::collections::HashMap;

use crate::data_types::{TimePeriod, User};
use crate::db::db_common;
use crate::db::errors::RowParsingError;

pub const DEFAULT_USER_ID: &str = "this is the default user id";

pub fn create_users_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE users (
            id TEXT PRIMARY KEY,
            created_time BIGINT,
            name TEXT NOT NULL,
            username TEXT NOT NULL,
            url TEXT,
            avatar_url TEXT NOT NULL,
            bio TEXT,
            banner_url TEXT,
            website TEXT,
            twitter_handle TEXT,
            discord_handle TEXT,
            is_bot BOOLEAN,
            is_admin BOOLEAN,
            is_trustworthy BOOLEAN,
            is_banned_from_posting BOOLEAN,
            user_deleted BOOLEAN,
            balance FLOAT NOT NULL,
            total_deposits FLOAT NOT NULL,
            last_bet_time BIGINT,
            current_betting_streak BIGINT,
            profit_cached TEXT
        );",
        [],
    )?;
    Ok(())
}

pub fn insert_user(conn: &mut Connection, user: User) -> Result<usize> {
    let stmt_str = "INSERT INTO users (
            id, created_time, name, username, url, avatar_url, bio, banner_url, website,
            twitter_handle, discord_handle, is_bot, is_admin, is_trustworthy,
            is_banned_from_posting, user_deleted, balance, total_deposits,
            last_bet_time, current_betting_streak, profit_cached
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9,
            ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18,
            ?19, ?20, ?21
        )";

    let tx = conn.transaction()?;

    {
        let mut stmt = tx.prepare(stmt_str)?;
        stmt.execute(params![
            user.id,
            user.created_time,
            user.name,
            user.username,
            user.url,
            user.avatar_url,
            user.bio,
            user.banner_url,
            user.website,
            user.twitter_handle,
            user.discord_handle,
            user.is_bot,
            user.is_admin,
            user.is_trustworthy,
            user.is_banned_from_posting,
            user.user_deleted,
            user.balance,
            user.total_deposits,
            user.last_bet_time,
            user.current_betting_streak,
            serde_json::to_string(&user.profit_cached).unwrap(),
        ])?;
    }
    tx.commit()?;

    Ok(1)
}

pub fn rusqlite_row_to_user(row: &Row) -> Result<User, RowParsingError> {
    let profit_cached_str: String = row.get(20)?;
    let profit_cached = serde_json::from_str::<HashMap<TimePeriod, f64>>(&profit_cached_str)?;

    Ok(User {
        id: row.get(0)?,
        created_time: row.get(1)?,
        name: row.get(2)?,
        username: row.get(3)?,
        url: row.get(4)?,
        avatar_url: row.get(5)?,
        bio: row.get(6)?,
        banner_url: row.get(7)?,
        website: row.get(8)?,
        twitter_handle: row.get(9)?,
        discord_handle: row.get(10)?,
        is_bot: row.get(11)?,
        is_admin: row.get(12)?,
        is_trustworthy: row.get(13)?,
        is_banned_from_posting: row.get(14)?,
        user_deleted: row.get(15)?,
        balance: row.get(16)?,
        total_deposits: row.get(17)?,
        last_bet_time: row.get(18)?,
        current_betting_streak: row.get(19)?,
        profit_cached,
    })
}

pub fn get_default_backtest_user() -> User {
    User {
        id: DEFAULT_USER_ID.to_string(),
        created_time: 1700000000000,
        name: "axel".to_string(),
        username: "axel".to_string(),
        url: Some("https://www.youtube.com/watch?v=NAh9oLs67Cw".to_string()),
        avatar_url: "https://upload.wikimedia.org/wikipedia/en/3/34/Garfield_the_Cat.jpg"
            .to_string(),
        bio: Some("blackness! darkness! black darkness, dark blankness!".to_string()),
        banner_url: Some("https://www.youtube.com/watch?v=NAh9oLs67Cw".to_string()),
        website: Some("https://www.youtube.com/watch?v=NAh9oLs67Cw".to_string()),
        twitter_handle: Some("https://www.youtube.com/watch?v=NAh9oLs67Cw".to_string()),
        discord_handle: Some("https://www.youtube.com/watch?v=NAh9oLs67Cw".to_string()),
        is_bot: Some(true),
        is_admin: Some(false),
        is_trustworthy: Some(false),
        is_banned_from_posting: Some(true),
        user_deleted: Some(false),
        balance: 1000.0,
        total_deposits: 0.0,
        last_bet_time: Some(1700000000001),
        current_betting_streak: Some(0),
        profit_cached: HashMap::<TimePeriod, f64>::from([
            (TimePeriod::Daily, 0.0),
            (TimePeriod::Weekly, 0.0),
            (TimePeriod::Monthly, 0.0),
            (TimePeriod::AllTime, 0.0),
        ]),
    }
}

pub fn init_user_table(conn: &mut Connection) -> Result<usize> {
    if !db_common::table_exists(conn, "users")? {
        debug!("creating 'users' table");
        create_users_table(conn)?;
    } else {
        debug!("found 'users' table");
    }

    let default_user = get_default_backtest_user();

    // see TODO comment in init_market_table
    let mut num_rows =
        db_common::count_rows(conn, "users").expect("failed to count rows in bets table");

    if num_rows == 0 {
        debug!("inserting the default user");
        insert_user(conn, default_user)?;
        num_rows = 1;
        debug!("1 user inserted...");
    } else {
        // TODO
        debug!("there are {num_rows} (instead of 0) rows, so not inserting anything");
    }

    Ok(num_rows)
}
