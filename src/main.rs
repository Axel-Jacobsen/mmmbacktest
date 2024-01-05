use warp::http::StatusCode;
use warp::Filter;

mod data_types;
mod db;

#[tokio::main]
async fn main() {
    db::setup_db();

    let root = warp::path::end().map(|| StatusCode::NOT_IMPLEMENTED);
    let base = warp::path("v0")
        .and(warp::path::end())
        .map(|| StatusCode::NOT_IMPLEMENTED);

    let routes = root.or(base);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
