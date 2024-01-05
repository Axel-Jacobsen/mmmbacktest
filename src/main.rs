use std::fs;

use warp::http::StatusCode;
use warp::Filter;

mod schema;
mod data_types;
mod db;

fn iter_over_markets(market_json: &String) -> Vec<data_types::FullMarket> {
    let file_as_string = fs::read_to_string(market_json).unwrap();
    let markets: Vec<data_types::FullMarket> = serde_json::from_str(&file_as_string).unwrap();
    markets
}

fn iterate_over_bets(bets_dir: &String) -> Vec<data_types::Bet> {
    let mut bets: Vec<data_types::Bet> = vec![];
    for file in fs::read_dir(bets_dir).unwrap() {
        let path = file.unwrap().path();

        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }

        let file_as_string = fs::read_to_string(path).unwrap();
        let inner_bets: Vec<data_types::Bet> =
            serde_json::from_str(&file_as_string).expect("failed to parse json into Bet struct");
        bets.extend(inner_bets);
    }
    bets
}

#[tokio::main]
async fn main() {
    let conn = db::setup_db();

    return;

    let markets =
        iter_over_markets(&"backtest-data/manifold-dump-markets-04082023.json".to_string());

    println!("found {:?} markets", markets.len());

    let bets = iterate_over_bets(&"backtest-data/bets".to_string());

    println!("found {:?} bets", bets.len());

    let root = warp::path::end().map(|| StatusCode::NOT_IMPLEMENTED);
    let base = warp::path("v0")
        .and(warp::path::end())
        .map(|| StatusCode::NOT_IMPLEMENTED);

    let routes = root.or(base);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
