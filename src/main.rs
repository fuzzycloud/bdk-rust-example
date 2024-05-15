mod setup;
use anyhow::Result;
use axum::{routing::get, Router};
mod app_error;

mod handler;
use crate::handler::handler;

#[tokio::main]
async fn main() -> Result<()> {
    let app = Router::new().route("/", get(handler));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
