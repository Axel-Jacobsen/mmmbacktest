use serde::{Deserialize, Serialize};
use std::{env, sync::Arc};
use warp::http::StatusCode;
use warp::Filter;

mod data_types;
mod db;

#[derive(Deserialize)]
struct MarketQueryParams {
    limit: Option<i64>,
    sort: Option<String>,
    order: Option<String>,
    before: Option<String>,
    user_id: Option<String>,
    _group_id: Option<String>,
}

#[derive(Serialize)]
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

    let connection_pool = Arc::new(db::setup_db());

    let root = warp::path::end().map(|| StatusCode::NOT_IMPLEMENTED);
    let v0 = warp::path("v0");
    let base = warp::path("v0")
        .and(warp::path::end())
        .map(|| StatusCode::NOT_IMPLEMENTED);

    // we have to clone this pool twice? I bet I got something wrong
    let connection_pool_clone = connection_pool.clone();
    let markets_endpoint = v0
        .and(warp::path("markets"))
        .and(warp::path::end())
        .and(warp::query::<MarketQueryParams>())
        .map(move |mq: MarketQueryParams| {
            let pool = connection_pool_clone.clone();
            let conn = pool
                .get()
                .expect("failed to get db connection from the pool");

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

    let connection_pool_clone = connection_pool.clone();
    let market_by_id = v0
        .and(warp::path("markets"))
        .and(warp::path::param())
        .and(warp::path::end())
        .map(move |market_id: String| {
            let pool = connection_pool_clone.clone();
            let conn = pool
                .get()
                .expect("failed to get db connection from the pool");

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

    let routes = root.or(base).or(markets_endpoint).or(market_by_id);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
