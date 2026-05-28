pub mod blizzard;
pub mod raiderio;
pub mod scraper;
pub mod warcraftlogs;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ApiConfig {
    pub blizzard_client_id: String,
    pub blizzard_client_secret: String,
    pub wcl_client_id: String,
    pub wcl_client_secret: String,
}

#[derive(Debug, Error)]
pub enum ApiError {
    #[error("invalid input: {0}")]
    InvalidInput(&'static str),
    #[error("expected API data was missing: {0}")]
    MissingData(String),
    #[error("HTTP request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("API integration is not implemented yet")]
    NotImplemented,
}

pub type ApiResult<T> = Result<T, ApiError>;
