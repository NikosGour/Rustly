mod constants;
mod handlers;
mod models;
mod routes;

use crate::constants::Env;
use crate::routes::app;
use models::Task;
use std::convert::Into;

fn seed_task() -> Vec<Task> {
    vec![
        Task {
            id: 0,
            title: "nikolis".into(),
            done: false,
        },
        Task {
            id: 1,
            title: "maroulio".into(),
            done: false,
        },
    ]
}
#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv().is_err_and(|err| panic!("on load dotenv: {err}"));
    let app = app();

    let listening_address = format!(
        "0.0.0.0:{}",
        dotenvy::var(Env::PORT.as_str()).unwrap_or(7000.to_string())
    );
    
    let listener = match tokio::net::TcpListener::bind(&listening_address).await {
        Ok(l) => l,
        Err(err) => panic!("on bind: {err}"),
    };

    println!("Listening on address: {listening_address}");

    if let Err(err) = axum::serve(listener, app).await {
        panic!("on serve: {err}",)
    }
}
