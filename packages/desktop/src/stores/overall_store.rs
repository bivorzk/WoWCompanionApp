use dioxus::prelude::*;

use crate::{
    services::{theme_for_class, LiveCompanionService, ThemeTokens},
    types::{CharacterProfileData, OverallSection},
    Route,
};

#[derive(Clone)]
pub(crate) struct OverallViewStore {
    pub(crate) selected_section: Signal<OverallSection>,
    pub(crate) profile: Resource<Result<CharacterProfileData, String>>,
    pub(crate) profile_link: Route,
    pub(crate) fallback_theme: ThemeTokens,
}

pub(crate) fn use_overall_view_store() -> OverallViewStore {
    let service = LiveCompanionService::new();
    let profile_link = service.sample_profile_route();
    let (region, realm, name) = match &profile_link {
        Route::CharacterProfile { region, realm, name } => {
            (region.clone(), realm.clone(), name.clone())
        }
        _ => (
            String::from("eu"),
            String::from("ravencrest"),
            String::from("Bvb"),
        ),
    };
    let profile = use_resource({
        let service = service.clone();
        let region = region.clone();
        let realm = realm.clone();
        let name = name.clone();

        move || {
            let service = service.clone();
            let region = region.clone();
            let realm = realm.clone();
            let name = name.clone();

            async move { service.character_profile(&region, &realm, &name).await }
        }
    });
    let selected_section = use_signal(|| OverallSection::Raids);

    OverallViewStore {
        selected_section,
        profile,
        profile_link,
        fallback_theme: theme_for_class(""),
    }
}