use dioxus::prelude::*;

use crate::{services::CompanionService, types::FavoriteCharacter, Route};

#[derive(Clone)]
pub(crate) struct FavoritesViewStore {
    pub(crate) demo_profile_link: Route,
    pub(crate) favorites: Signal<Vec<FavoriteCharacter>>,
}

pub(crate) fn use_favorites_view_store() -> FavoritesViewStore {
    let service = CompanionService::mock();
    let demo_profile_link = service.sample_profile_route();
    let favorites = use_signal(move || service.favorite_profiles());

    FavoritesViewStore {
        demo_profile_link,
        favorites,
    }
}