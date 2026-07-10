use axum::{routing::get, Router};

mod routers;
use routers::{get_names, get_peaks};

#[tokio::main]
async fn main() {
    let router = Router::new()
    .route("/list",get(get_names))
    .route("/peaks/{sample}",get(get_peaks));

    let address: &'static str = "0.0.0.0:8000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
