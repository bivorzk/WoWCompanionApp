use crate::ApiResult;

#[derive(Clone, Debug, Default)]
pub struct RaiderIoClient;

impl RaiderIoClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn fetch_character_profile(
        &self,
        _region: &str,
        _realm: &str,
        _name: &str,
    ) -> ApiResult<()> {
        Ok(())
    }
}