use crate::data_types::Bet;
use crate::db::db_common::*;
use log::debug;
use rusqlite::{params, Connection, Result};
use std::fs;

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

pub fn create_bet_table(conn: &Connection) -> Result<()> {
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
            ?21, ?22, ?23
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

pub fn init_bet_table(conn: &mut Connection) -> Result<usize> {
    if !table_exists(conn, "bets")? {
        debug!("creating 'bets' table");
        create_bet_table(conn)?;
    }

    let mut count = 0;

    // see TODO comment in init_market_table
    if count_rows(conn, "bets").expect("failed to count rows in bets table") == 0 {
        debug!("inserting bets");

        for bets in iter_over_bets("backtest-data/bets") {
            count += bulk_insert_bets(conn, &bets)?;
        }

        debug!("{count} bets inserted...");
    }

    Ok(count)
}
