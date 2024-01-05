use rusqlite::{params, Connection, Result};

use std::fs;

use crate::data_types::{Bet, FullMarket, LiteMarket};

// https://chat.openai.com/share/d2e2e7a3-80ed-4502-b090-9eb6237c5c74
// Geez this is harder than I thought. I hope that throwing it all into a sqlite db
// won't be too painful. I wish I knew more about this. Enough for today. Why is sql
// serialization so ugly?? I hate that I have to go convert json to a rust struct to
// convert it to a sqlite struct, just to be converted back to a rust struct and then
// serialized back into json. That's awful.
//
// Actually, there *has* to be a better way. This is insane. Maybe just making my own index is
// fine.

fn iter_over_markets(market_json: &String) -> Vec<FullMarket> {
    let file_as_string = fs::read_to_string(market_json).unwrap();
    let markets: Vec<FullMarket> = serde_json::from_str(&file_as_string).unwrap();
    markets
}

// fn iterate_over_bets(bets_dir: &String) -> Vec<Bet> {
//     let mut bets: Vec<Bet> = vec![];

//     for file in fs::read_dir(bets_dir).unwrap() {
//         let path = file.unwrap().path();

//         if path.extension().and_then(|s| s.to_str()) != Some("json") {
//             continue;
//         }

//         let file_as_string = fs::read_to_string(path).unwrap();
//         let inner_bets: Vec<Bet> =
//             serde_json::from_str(&file_as_string).expect("failed to parse json into Bet struct");
//         bets.extend(inner_bets);
//     }

//     bets
// }

fn iterate_over_bets(bets_dir: &str) -> impl Iterator<Item = Vec<Bet>> {
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

fn create_bet_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE bets (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            user_avatar_url TEXT,
            user_name TEXT,
            user_username TEXT,
            contract_id TEXT NOT NULL,
            answer_id TEXT,
            created_time BIGINT NOT NULL,
            amount FLOAT NOT NULL,
            loan_amount FLOAT,
            outcome TEXT NOT NULL,
            shares FLOAT NOT NULL,
            prob_before FLOAT NOT NULL,
            prob_after FLOAT NOT NULL,
            fees TEXT,
            is_api BOOL,
            is_ante BOOL NOT NULL,
            is_redemption BOOL NOT NULL,
            is_challenge BOOL NOT NULL,
            visibility TEXT,
            challenge_slug TEXT,
            reply_to_comment_id TEXT,
            limit_props TEXT
        )",
        [],
    )?;
    Ok(())
}

fn create_litemarket_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE markets (
            id TEXT PRIMARY KEY,
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

pub fn bulk_insert_bets(conn: &mut Connection, bets: &Vec<Bet>) -> Result<usize> {
    let stmt_str = "INSERT INTO bets (
            id, user_id, user_avatar_url, user_name, user_username, contract_id, answer_id,
            created_time, amount, loan_amount, outcome, shares,
            prob_before, prob_after, fees, is_api, is_ante, is_redemption, is_challenge,
            visibility, challenge_slug, reply_to_comment_id, limit_props
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7,
            ?8, ?9, ?10, ?11, ?12, ?13,
            ?14, ?15, ?16, ?17, ?18, ?19, ?20,
            ?21, ?22, ?23, ?24
        )";

    for chunk in bets.chunks(1000) {
        let tx = conn.transaction()?;

        {
            // scope so tx borrow (from prepare) is OK
            let mut stmt = tx.prepare(stmt_str)?;
            for bet in chunk {
                stmt.execute(params![
                    bet.id,
                    bet.user_id,
                    bet.user_avatar_url,
                    bet.user_name,
                    bet.user_username,
                    bet.contract_id,
                    bet.answer_id,
                    bet.created_time,
                    bet.amount,
                    bet.loan_amount,
                    bet.outcome,
                    bet.shares,
                    bet.prob_before,
                    bet.prob_after,
                    serde_json::to_string(&bet.fees).unwrap(),
                    bet.is_api,
                    bet.is_ante,
                    bet.is_redemption,
                    bet.is_challenge,
                    serde_json::to_string(&bet.visibility).unwrap(),
                    bet.challenge_slug,
                    bet.reply_to_comment_id,
                    serde_json::to_string(&bet.limit_props).unwrap()
                ])?;
            }
        }

        tx.commit()?;
    }

    Ok(bets.len())
}
pub fn bulk_insert_markets(conn: &mut Connection, markets: &Vec<LiteMarket>) -> Result<usize> {
    let stmt_str = "INSERT INTO markets (
        id, creator_username, creator_name, creator_avatar_url, close_time,
        created_time, question, url, outcome_type, mechanism, probability,
        pool, p, total_liquidity, value, min, max, is_log_scale, volume,
        volume_24_hours, is_resolved, resolution_time, resolution,
        resolution_probability, last_updated_time, last_bet_time
    ) VALUES (
        ?1, ?2, ?3, ?4, ?5,
        ?6, ?7, ?8, ?9, ?10, ?11,
        ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19,
        ?20, ?21, ?22, ?23,
        ?24, ?25, ?26
    );";

    for chunk in markets.chunks(1000) {
        let tx = conn.transaction()?;

        {
            // scope so tx borrow (from prepare) is OK
            let mut stmt = tx.prepare(stmt_str)?;
            for market in chunk {
                stmt.execute(params![
                    market.id,
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

fn create_bets_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE Bet (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL,
            user_avatar_url TEXT,
            user_name TEXT,
            user_username TEXT,
            contract_id TEXT NOT NULL,
            answer_id TEXT,
            created_time INTEGER NOT NULL,
            amount REAL NOT NULL,
            loan_amount REAL,
            outcome TEXT NOT NULL,
            shares REAL NOT NULL,
            shares_by_outcome TEXT,
            prob_before REAL NOT NULL,
            prob_after REAL NOT NULL,
            fees TEXT,
            is_api INTEGER,
            is_ante INTEGER NOT NULL,
            is_redemption INTEGER NOT NULL,
            is_challenge INTEGER NOT NULL,
            visibility TEXT NOT NULL,
            challenge_slug TEXT,
            reply_to_comment_id TEXT,
            limit_props TEXT
        );",
        [],
    )?;
    Ok(())
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
    if !table_exists(&conn, "markets")? {
        println!("creating markets table");
        create_litemarket_table(&conn)?;
    }

    let mut count = 0;

    if count_rows(&conn, "markets").expect("failed to count rows in markets table") == 0 {
        println!("inserting markets");

        let markets =
            iter_over_markets(&"backtest-data/manifold-dump-markets-04082023.json".to_string());

        let lite_markets: Vec<LiteMarket> =
            markets.iter().map(|fm| fm.lite_market.clone()).collect();

        count = bulk_insert_markets(conn, &lite_markets)?;
    }

    Ok(count)
}

fn init_bet_table(conn: &mut Connection) -> Result<usize> {
    if !table_exists(&conn, "bets")? {
        create_bet_table(&conn)?;
    }

    let mut count = 0;

    if count_rows(&conn, "bets").expect("failed to count rows in bets table") == 0 {
        println!("inserting bets");

        for bets in iterate_over_bets(&"backtest-data/bets".to_string()) {
            count += bulk_insert_bets(conn, &bets)?;
        }
    }

    Ok(count)
}

pub fn setup_db() -> Connection {
    let mut conn = get_db_connection().expect("failed to get db connection");

    init_market_table(&mut conn).expect("failed to init market table");
    init_bet_table(&mut conn).expect("failed to init bet table");

    conn
}
