use std::net::SocketAddr;

use axum::{Router};
use axum::routing::get;

use crate::controller::hello_controller::hello;

pub mod controller;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/hello", get(hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
