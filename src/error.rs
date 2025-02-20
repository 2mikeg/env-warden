use rusqlite::Error as RusqliteError;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("Rusqlite Error: {0}")]
    Rusqlite(#[from] RusqliteError),
}
