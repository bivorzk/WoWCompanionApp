use crate::{ApiConfig, ApiResult};

#[derive(Clone, Debug, Default)]
pub struct WarcraftLogsClient;

impl WarcraftLogsClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn authenticate(&self, _config: &ApiConfig) -> ApiResult<()> {
        Ok(())
    }
}