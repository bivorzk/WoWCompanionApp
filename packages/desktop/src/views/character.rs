use dioxus::prelude::*;

use crate::{
    components::{
        AppShell, CharacterHeader, CollectionTab, EndgameTab, MythicTab, ProfileSidebar,
        PvpTab, RaidsTab, TabBar,
    },
    services::theme_for_class,
    stores::use_character_view_store,
    types::{NavSection, ProfileTab},
};

#[component]
pub(crate) fn CharacterProfile(region: String, realm: String, name: String) -> Element {
    let store = use_character_view_store(region, realm, name);
    let (profile, error, theme) = {
        let profile_state = store.profile.read_unchecked();

        match &*profile_state {
            Some(Ok(profile)) => (
                Some(profile.clone()),
                None,
                theme_for_class(&profile.class_name),
            ),
            Some(Err(error)) => (None, Some(error.clone()), store.fallback_theme),
            None => (None, None, store.fallback_theme),
        }
    };

    rsx! {
        AppShell {
            current: NavSection::Character,
            profile_link: store.route.clone(),
            accent: theme.accent,
            accent_muted: theme.accent_muted,
            if let Some(profile) = profile {
                div { class: "profile-root",
                    CharacterHeader { profile: profile.clone() }
                    div { class: "profile-layout",
                        aside { class: "profile-sidebar",
                            ProfileSidebar { profile: profile.clone() }
                        }
                        section { class: "profile-main",
                            TabBar { active_tab: store.active_tab }
                            div { class: "tab-panel",
                                if (store.active_tab)() == ProfileTab::Pvp {
                                    PvpTab { profile: profile.clone() }
                                }
                                if (store.active_tab)() == ProfileTab::Raids {
                                    RaidsTab { profile: profile.clone() }
                                }
                                if (store.active_tab)() == ProfileTab::Mythic {
                                    MythicTab { profile: profile.clone() }
                                }
                                if (store.active_tab)() == ProfileTab::Endgame {
                                    EndgameTab { profile: profile.clone() }
                                }
                                if (store.active_tab)() == ProfileTab::Collection {
                                    CollectionTab { profile }
                                }
                            }
                        }
                    }
                }
            } else if let Some(error) = error {
                section { class: "section-card card-elevated",
                    div { class: "section-heading",
                        h1 { class: "section-title", "Live Character Unavailable" }
                        p { class: "text-muted",
                            "The character route now loads from live APIs, but the current profile request failed."
                        }
                    }
                    div { class: "legend-card card-surface",
                        h2 { "Request error" }
                        p { class: "text-muted", "{error}" }
                    }
                }
            } else {
                section { class: "section-card card-elevated",
                    div { class: "section-heading",
                        h1 { class: "section-title", "Loading Live Character" }
                        p { class: "text-muted",
                            "Fetching live Raider.IO and Blizzard data for the active profile."
                        }
                    }
                }
            }
        }
    }
}