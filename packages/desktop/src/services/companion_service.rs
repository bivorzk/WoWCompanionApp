use crate::{
    repositories::{CompanionRepository, MockCompanionRepository},
    types::FavoriteCharacter,
    Route,
};

#[derive(Clone, Copy)]
pub(crate) struct CompanionService<R = MockCompanionRepository> {
    repository: R,
}

impl CompanionService<MockCompanionRepository> {
    pub(crate) fn mock() -> Self {
        Self {
            repository: MockCompanionRepository,
        }
    }
}

impl<R> CompanionService<R>
where
    R: CompanionRepository,
{
    pub(crate) fn sample_profile_route(&self) -> Route {
        self.repository.sample_profile_route()
    }

    pub(crate) fn favorite_profiles(&self) -> Vec<FavoriteCharacter> {
        self.repository.favorite_profiles()
    }
}