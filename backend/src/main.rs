use axum::{routing::get, Router};
use std::sync::Arc;

mod state;
mod models;
mod handlers;

use state::AppState;
use handlers::{get_files, get_peaks};


#[tokio::main]
async fn main() {
    let state = AppState{
        db_path: Arc::new("../results/peaks.db".to_string()),
    };

    let router = Router::new()
    .route("/list",get(get_files))
    .route("/peaks/{sample}",get(get_peaks))
    .with_state(state);

    let address: &'static str = "0.0.0.0:8000";
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();

    axum::serve(listener, router).await.unwrap();
}
