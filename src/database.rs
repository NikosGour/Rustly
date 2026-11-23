use std::error::Error;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::constants::Env;

pub async  fn setup_database() -> Result<Pool<Postgres>, Box<dyn Error>>{
    let max_connections = dotenvy::var(Env::PostgresMaxConnections.as_str())
        .unwrap_or(5.to_string())
        .parse()?;
    let db_username =
        dotenvy::var(Env::PostgresRootUsername.as_str()).unwrap_or("postgres".to_string());
    let db_password = dotenvy::var(Env::PostgresRootPassword.as_str())
        .expect(Env::PostgresRootPassword.not_defined().as_str());
    let db_database_name =
        dotenvy::var(Env::PostgresDb.as_str()).expect(Env::PostgresDb.not_defined().as_str());
    let db_url =
        dotenvy::var(Env::PostgresUrl.as_str()).expect(Env::PostgresUrl.not_defined().as_str());

    let connection_string =
        format!("postgres://{db_username}:{db_password}@{db_url}/{db_database_name}");

     Ok(PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(&connection_string)
        .await?)
}