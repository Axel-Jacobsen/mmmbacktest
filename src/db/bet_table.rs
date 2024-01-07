use indicatif::{ProgressBar, ProgressStyle};
use log::debug;
use rusqlite::{params, Connection, Result, Row};
use std::fs;

use crate::data_types::Bet;
use crate::data_types::{Fees, LimitProps, Visibility};
use crate::db::db_common;
use crate::db::errors::RowParsingError;

fn iter_over_bets(bets_dir: &str) -> impl Iterator<Item = Vec<Bet>> {
    let paths = fs::read_dir(bets_dir).unwrap_or_else(|err| {
        panic!("Error reading directory: {}", err);
    });

    // Count the number of files that will be processed
    let total_files = paths
        .filter(|entry| {
            if let Ok(e) = entry {
                let path = e.path();
                path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json")
            } else {
                false
            }
        })
        .count();

    // Create a new progress bar with the total count
    let progress_bar = ProgressBar::new(total_files as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} ({eta})",
            )
            .unwrap()
            .progress_chars("#>-"),
    );

    // Reset the iterator to read the directory again
    let paths = fs::read_dir(bets_dir).unwrap();

    paths.filter_map(move |entry| {
        if let Ok(e) = entry {
            let path = e.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                let file_as_string =
                    fs::read_to_string(&path).expect("couldn't read file to string");

                progress_bar.inc(1); // Increment the progress bar

                serde_json::from_str(&file_as_string).expect("failed to parse json into Bet struct")
            } else {
                None
            }
        } else {
            None
        }
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

/// Attempts to convert row into a Bet.
/// If there's the wrong number of rows, we return an Err.
/// Sort-of an inverse of bulk_insert_markets
pub fn rusqlite_row_to_bet(row: &Row) -> Result<Bet, RowParsingError> {
    let fees_str: String = row.get(14)?;
    let visibility_str: String = row.get(19)?;
    let limit_props_str: String = row.get(22)?;

    let fees = if fees_str == "null" {
        None
    } else {
        Some(serde_json::from_str::<Fees>(&fees_str)?)
    };
    let visibility = serde_json::from_str::<Visibility>(&visibility_str)?;
    let limit_props_str = if limit_props_str == "null" {
        None
    } else {
        Some(serde_json::from_str::<LimitProps>(&limit_props_str)?)
    };

    Ok(Bet {
        id: row.get(0)?,
        user_id: row.get(1)?,
        user_avatar_url: row.get(2)?,
        user_name: row.get(3)?,
        user_username: row.get(4)?,
        contract_id: row.get(5)?,
        answer_id: row.get(6)?,
        created_time: row.get(7)?,
        amount: row.get(8)?,
        loan_amount: row.get(9)?,
        outcome: row.get(10)?,
        shares: row.get(11)?,
        shares_by_outcome: None,
        prob_before: row.get(12)?,
        prob_after: row.get(13)?,
        fees,
        is_api: row.get(15)?,
        is_ante: row.get(16)?,
        is_redemption: row.get(17)?,
        is_challenge: row.get(18)?,
        visibility,
        challenge_slug: row.get(20)?,
        reply_to_comment_id: row.get(21)?,
        limit_props: limit_props_str,
    })
}

pub fn init_bet_table(conn: &mut Connection) -> Result<usize> {
    if !db_common::table_exists(conn, "bets")? {
        debug!("creating 'bets' table");
        create_bet_table(conn)?;
    } else {
        debug!("found 'bets' table");
    }

    let mut count = 0;

    // see TODO comment in init_market_table
    let num_rows = db_common::count_rows(conn, "bets").expect("failed to count rows in bets table");
    if num_rows == 0 {
        debug!("inserting bets");

        for bets in iter_over_bets("backtest-data/bets") {
            count += bulk_insert_bets(conn, &bets)?;
        }

        debug!("{count} bets inserted...");
    } else {
        // TODO
        debug!("there are {num_rows} (instead of 0) rows, so not inserting anything");
    }

    Ok(count)
}
