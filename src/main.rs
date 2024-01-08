use serde::{Deserialize, Serialize};
use std::env;
use warp::{http::StatusCode, Filter};

mod data_types;
mod db;

use crate::db::db_common::{get_db_connection, setup_db};

#[derive(Deserialize)]
struct MarketQueryParams {
    limit: Option<i64>,
    sort: Option<String>,
    order: Option<String>,
    before: Option<String>,
    #[serde(rename = "userId")]
    user_id: Option<String>,
    _group_id: Option<String>,
}

#[derive(Deserialize)]
struct BetQueryParams {
    #[serde(rename = "userId")]
    user_id: Option<String>,
    username: Option<String>,
    #[serde(rename = "contractId")]
    contract_id: Option<String>,
    #[serde(rename = "contractSlug")]
    contract_slug: Option<String>,
    limit: Option<i64>,
    before: Option<String>,
    after: Option<String>,
    order: Option<String>,
}

#[derive(Debug, Serialize)]
struct HttpError {
    code: u16,
    message: String,
}

fn ret_http_error(code: u16, message: String) -> warp::reply::Json {
    log::error!("{}", message);
    warp::reply::json(&HttpError { code, message })
}

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let connection_pool = setup_db();

    let root = warp::path::end().map(|| StatusCode::NOT_IMPLEMENTED);
    let v0 = warp::path("v0");
    let base = warp::path("v0")
        .and(warp::path::end())
        .map(|| StatusCode::NOT_IMPLEMENTED);

    // MAJOR TODO
    //
    // Listing out all these endpoints is messy. I want to move the
    // endpoint construction somewhere else. How do we do this?
    //
    // we have to clone this pool twice? I bet I got something wrong
    let connection_pool_clone = connection_pool.clone();
    let markets_endpoint = v0
        .and(warp::path("markets"))
        .and(warp::path::end())
        .and(warp::query::<MarketQueryParams>())
        .map(move |mq: MarketQueryParams| {
            let conn = get_db_connection(connection_pool_clone.clone());

            let maybe_markets = db::get_markets_by_params(
                &conn,
                mq.limit,
                mq.sort.as_deref(),
                mq.order.as_deref(),
                mq.before.as_deref(),
                mq.user_id.as_deref(),
            );

            match maybe_markets {
                Ok(markets) => {
                    log::info!("returning {} markets", markets.len());
                    warp::reply::json(&markets)
                }
                Err(e) => ret_http_error(400, e.to_string()),
            }
        });

    // we return a LiteMarket instead of a FullMarket here :(
    let connection_pool_clone = connection_pool.clone();
    let market_by_id_endpoint = v0
        .and(warp::path("markets"))
        .and(warp::path::param())
        .and(warp::path::end())
        .map(move |market_id: String| {
            let conn = get_db_connection(connection_pool_clone.clone());

            let maybe_markets = db::get_markets_by_id(&conn, Some(market_id.as_str()));

            let markets = match maybe_markets {
                Ok(markets) => markets,
                Err(e) => return ret_http_error(400, e.to_string()),
            };

            if markets.is_empty() {
                ret_http_error(400, format!("no markets found for market id {market_id}"))
            } else if markets.len() > 1 {
                ret_http_error(
                    400,
                    format!("more than one market found for market id {market_id}"),
                )
            } else {
                log::info!("returning market with id {market_id}");
                warp::reply::json(&markets[0])
            }
        });

    let connection_pool_clone = connection_pool.clone();
    let bets_endpoint = v0
        .and(warp::path("bets"))
        .and(warp::path::end())
        .and(warp::query::<BetQueryParams>())
        .map(move |bq: BetQueryParams| {
            let conn = get_db_connection(connection_pool_clone.clone());

            let maybe_bets = db::get_bets_by_params(
                &conn,
                bq.user_id.as_deref(),
                bq.username.as_deref(),
                bq.contract_id.as_deref(),
                bq.contract_slug.as_deref(),
                bq.limit,
                bq.before.as_deref(),
                bq.after.as_deref(),
                bq.order.as_deref(),
            );

            match maybe_bets {
                Ok(bets) => {
                    log::info!("returning {} bets", bets.len());
                    warp::reply::json(&bets)
                }
                Err(e) => ret_http_error(400, e.to_string()),
            }
        });

    let connection_pool_clone = connection_pool.clone();
    let market_by_slug_endpoint = v0
        .and(warp::path("slug"))
        .and(warp::path::param())
        .and(warp::path::end())
        .map(move |slug: String| {
            let conn = get_db_connection(connection_pool_clone.clone());

            let maybe_markets = db::get_market_by_slug(&conn, slug.as_str());

            let markets = match maybe_markets {
                Ok(markets) => markets,
                // TODO probably should not give the error string to the user
                // bad security practice?
                Err(e) => return ret_http_error(400, e.to_string()),
            };

            if markets.is_empty() {
                ret_http_error(400, format!("no markets found for slug {slug}"))
            } else if markets.len() > 1 {
                ret_http_error(400, format!("more than one market found for slug {slug}"))
            } else {
                log::info!("returning market with slug {slug}");
                warp::reply::json(&markets[0])
            }
        });

    let connection_pool_clone = connection_pool.clone();
    let me_endpoint = v0
        .and(warp::path("me"))
        .and(warp::path::end())
        .map(move || {
            let conn = get_db_connection(connection_pool_clone.clone());

            let me = db::get_me(&conn);

            match me {
                Ok(me) => warp::reply::json(&me),
                Err(_) => ret_http_error(400, "couldn't return user".to_string()),
            }
        });

    let routes = root
        .or(base)
        .or(markets_endpoint)
        .or(market_by_id_endpoint)
        .or(bets_endpoint)
        .or(market_by_slug_endpoint)
        .or(me_endpoint);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
