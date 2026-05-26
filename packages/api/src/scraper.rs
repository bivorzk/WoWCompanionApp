use crate::ApiResult;

#[derive(Clone, Debug, Default)]
pub struct ScraperClient;

impl ScraperClient {
    pub fn new() -> Self {
        Self
    }

    pub async fn fetch_document(&self, _url: &str) -> ApiResult<::scraper::Html> {
        Ok(::scraper::Html::parse_document(""))
    }
}