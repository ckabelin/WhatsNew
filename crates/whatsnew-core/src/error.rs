use thiserror::Error;

#[derive(Debug, Error)]
pub enum CoreError {
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("migration error: {0}")]
    Migration(#[from] sqlx::migrate::MigrateError),

    #[error("http error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("feed parse error: {0}")]
    FeedParse(#[from] feed_rs::parser::ParseFeedError),

    #[error("topic {0} not found")]
    TopicNotFound(i64),

    #[error("feed {0} not found")]
    FeedNotFound(i64),

    #[error("invalid input: {0}")]
    InvalidInput(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;
