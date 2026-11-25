use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Database error")]
    Database(#[from] sqlx::Error),
}
