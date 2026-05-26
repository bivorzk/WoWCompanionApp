use crate::{ApiConfig, ApiResult};

#[derive(Clone, Debug, Default)]
pub struct BlizzardClient;

impl BlizzardClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn authenticate(&self, _config: &ApiConfig) -> ApiResult<()> {
        Ok(())
    }
}