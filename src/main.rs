use serde::Deserialize;
use std::env;
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
    group_id: Option<String>,
}

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let connection_pool = db::setup_db();

    let root = warp::path::end().map(|| StatusCode::NOT_IMPLEMENTED);
    let base = warp::path("v0")
        .and(warp::path::end())
        .map(|| StatusCode::NOT_IMPLEMENTED);

    let markets_endpoint = warp::path("v0")
        .and(warp::path("markets"))
        .and(warp::path::end())
        .and(warp::query::<MarketQueryParams>())
        .map(move |mq: MarketQueryParams| {
            let conn = connection_pool
                .get()
                .expect("failed to get db connection from the pool");

            println!(
                "{:?}",
                db::get_markets(
                    &conn,
                    mq.limit,
                    mq.sort.as_deref(),
                    mq.order.as_deref(),
                    mq.before.as_deref(),
                    mq.user_id.as_deref(),
                    mq.group_id.as_deref(),
                )
            );
            // placeholder until I figure out the return type from db::get_markets
            StatusCode::NOT_IMPLEMENTED
        });

    let routes = root.or(base).or(markets_endpoint);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
