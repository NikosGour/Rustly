mod config;
mod controllers;
mod dtos;
pub mod errors;
mod models;
mod repositories;
mod services;

use config::constants::Env;
use config::database;
use config::routes::app;
use std::error::Error;

use config::state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let _ = dotenvy::dotenv().expect("on load dotenv");
    let pool = database::setup_database().await?;

    let app = app(AppState::new(pool));
    let listening_address = format!(
        "0.0.0.0:{}",
        dotenvy::var(Env::Port.as_str()).unwrap_or(7000.to_string())
    );

    let listener = tokio::net::TcpListener::bind(&listening_address).await?;

    println!("Listening on address: {listening_address}");

    axum::serve(listener, app).await.expect("on serve");

    Ok(())
}
