use warp::http::StatusCode;
use warp::Filter;

#[tokio::main]
async fn main() {
    let root = warp::path::end().map(|| StatusCode::NOT_IMPLEMENTED);
    let base = warp::path("v0").map(|| StatusCode::NOT_IMPLEMENTED);

    let routes = root.or(base);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
