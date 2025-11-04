use axum::{Router, routing};
use tokio::net;

#[tokio::main]
async fn main() {
    architecture::run_app().await;

    let router = Router::new().route("/", routing::get(|| async { "Hello, World!" }));

    // run our app with hyper, listening globally on port 3000
    let listener = net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
