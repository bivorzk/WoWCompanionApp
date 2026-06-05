use crate::{repositories::LiveCompanionRepository, types::CharacterProfileData, Route};

#[derive(Clone, Default)]
pub(crate) struct LiveCompanionService {
    repository: LiveCompanionRepository,
}

impl LiveCompanionService {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn sample_profile_route(&self) -> Route {
        self.repository.sample_profile_route()
    }

    pub(crate) async fn character_profile(
        &self,
        region: &str,
        realm: &str,
        name: &str,
    ) -> Result<CharacterProfileData, String> {
        self.repository.character_profile(region, realm, name).await
    }
}