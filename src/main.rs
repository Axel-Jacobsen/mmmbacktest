use std::fs;

use warp::http::StatusCode;
use warp::Filter;

mod db;
mod data_types;


fn iter_over_markets(market_json: &String) -> Vec<data_types::LiteMarket> {
    let file_as_string = fs::read_to_string(market_json).unwrap();
    let markets: Vec<data_types::LiteMarket> = serde_json::from_str(&file_as_string).unwrap();
    markets
}

fn iterate_over_bets(bets_dir: &String) -> Vec<data_types::Bet> {
    let mut bets: Vec<data_types::Bet> = vec![];
    for file in fs::read_dir(bets_dir).unwrap() {
        println!("{:?}", file);
        let path = file.unwrap().path();

        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            println!("continuing with {:?}", path);
            continue;
        }

        let file_as_string = fs::read_to_string(path).unwrap();
        let inner_bets: Vec<data_types::Bet> = serde_json::from_str(&file_as_string).unwrap();
        println!("{:?}", inner_bets.len());
        bets.extend(inner_bets);
    }
    bets
}


#[tokio::main]
async fn main() {
    let markets = iter_over_markets(&"backtest-data/manifold-dump-markets-04082023.json".to_string());

    for market in markets {
        println!("{:?}", market);
    }

    let bets = iterate_over_bets(&"backtest-data/bets".to_string());

    for bet in bets {
        println!("{:?}", bet);
    }

    let root = warp::path::end().map(|| StatusCode::NOT_IMPLEMENTED);
    let base = warp::path("v0")
        .and(warp::path::end())
        .map(|| StatusCode::NOT_IMPLEMENTED);

    let routes = root.or(base);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
