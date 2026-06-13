use std::path::Path;
use std::str::FromStr;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;

use crate::error::Result;

pub mod articles;
pub mod feeds;
pub mod settings;
pub mod topics;

/// A connected, migrated SQLite database.
#[derive(Clone)]
pub struct Db {
    pub pool: SqlitePool,
}

impl Db {
    /// Opens (creating if necessary) the database file at `path` and runs migrations.
    pub async fn connect(path: &Path) -> Result<Self> {
        let opts = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true);
        let db = Self::from_options(opts).await?;
        Ok(db)
    }

    /// Opens an in-memory database and runs migrations. Used in tests.
    pub async fn connect_in_memory() -> Result<Self> {
        let opts = SqliteConnectOptions::from_str("sqlite::memory:")?;
        Self::from_options(opts).await
    }

    async fn from_options(opts: SqliteConnectOptions) -> Result<Self> {
        let pool = SqlitePoolOptions::new()
            // SQLite only supports one writer at a time; a single connection avoids
            // "database is locked" errors under the app's modest concurrency needs.
            .max_connections(1)
            .connect_with(opts)
            .await?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(Self { pool })
    }
}
