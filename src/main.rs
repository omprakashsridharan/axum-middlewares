use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get, Router};
use std::{net::SocketAddr, time::Duration};
use tokio::time::sleep;
use tower::{BoxError, ServiceBuilder};

use crate::timeout::TimeoutLayer;

mod timeout;

// basic handler that responds with a static string
async fn root() -> &'static str {
    sleep(Duration::from_secs(3)).await;
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new().route("/", get(root)).layer(
        ServiceBuilder::new()
            .layer(HandleErrorLayer::new(|_: BoxError| async {
                StatusCode::REQUEST_TIMEOUT
            }))
            .layer(TimeoutLayer::new(Duration::from_secs(2))),
    );
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
