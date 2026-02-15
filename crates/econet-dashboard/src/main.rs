mod api;
mod storage;

use std::net::SocketAddr;

use axum::Router;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let app: Router = api::app();
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("EcoNet dashboard API listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
