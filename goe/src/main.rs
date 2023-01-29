mod goe;
mod prom;


#[tokio::test]
async fn test_status() {
    let status = status().await.unwrap();
    assert!(status.energy() > 0)
}



use axum::{
    routing::get,
    Router, Json,
};
use goe::{status, Status};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/status", get(goe_status))
        .route("/metrics", get(metrics));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn goe_status() -> Json<Status> {
    let status = status().await.unwrap();
    Json(status)
}

async fn metrics() -> String {
    let status = status().await.unwrap();
    prom::metrics(status)
}