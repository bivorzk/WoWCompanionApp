use dioxus::prelude::*;

use crate::{
    services::{theme_for_class, LiveCompanionService, ThemeTokens},
    types::{CharacterProfileData, ProfileTab},
    Route,
};

#[derive(Clone)]
pub(crate) struct CharacterViewStore {
    pub(crate) active_tab: Signal<ProfileTab>,
    pub(crate) profile: Resource<Result<CharacterProfileData, String>>,
    pub(crate) route: Route,
    pub(crate) fallback_theme: ThemeTokens,
}

pub(crate) fn use_character_view_store(
    region: String,
    realm: String,
    name: String,
) -> CharacterViewStore {
    let service = LiveCompanionService::new();
    let route = Route::CharacterProfile {
        region: region.clone(),
        realm: realm.clone(),
        name: name.clone(),
    };
    let profile = use_resource(use_reactive(
        (&region, &realm, &name),
        move |(region, realm, name)| {
            let service = service.clone();

            async move { service.character_profile(&region, &realm, &name).await }
        },
    ));
    let active_tab = use_signal(|| ProfileTab::Pvp);

    CharacterViewStore {
        active_tab,
        profile,
        route,
        fallback_theme: theme_for_class(""),
    }
}