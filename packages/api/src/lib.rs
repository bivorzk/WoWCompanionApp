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
    #[error("API integration is not implemented yet")]
    NotImplemented,
}

pub type ApiResult<T> = Result<T, ApiError>;
