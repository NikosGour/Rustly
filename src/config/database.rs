use crate::config::constants::Env;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::error::Error;

pub async fn setup_database() -> Result<Pool<Postgres>, Box<dyn Error>> {
    let max_connections = dotenvy::var(Env::PostgresMaxConnections.as_str())
        .unwrap_or(5.to_string())
        .parse()?;
    let connection_string = dotenvy::var(Env::DatabaseUrl.as_str())
        .unwrap_or_else(|_| panic!("{}", Env::DatabaseUrl.not_defined()));

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&connection_string)
        .await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(pool)
}
