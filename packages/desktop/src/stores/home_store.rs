use dioxus::prelude::*;

use crate::{services::CompanionService, types::FavoriteCharacter, Route};

#[derive(Clone)]
pub(crate) struct HomeViewStore {
    pub(crate) demo_profile_link: Route,
    pub(crate) favorites: Signal<Vec<FavoriteCharacter>>,
}

pub(crate) fn use_home_view_store() -> HomeViewStore {
    let service = CompanionService::mock();
    let demo_profile_link = service.sample_profile_route();
    let favorites = use_signal(move || service.favorite_profiles());

    HomeViewStore {
        demo_profile_link,
        favorites,
    }
}