use dioxus::prelude::*;

use crate::{
    components::{AppShell, FavoriteCard},
    stores::use_favorites_view_store,
    types::NavSection,
};

#[component]
pub(crate) fn Favorites() -> Element {
    let store = use_favorites_view_store();
    let demo = store.demo_profile_link.clone();
    let favorites = (store.favorites)();

    rsx! {
        AppShell {
            current: NavSection::Favorites,
            profile_link: demo,
            accent: "#AAD372",
            accent_muted: "rgba(170,211,114,0.15)",
            section { class: "section-card card-elevated",
                div { class: "section-heading",
                    h1 { class: "section-title", "Favorites" }
                    p { class: "text-muted",
                        "Pinned profiles keep the desktop shell useful even before any live data plumbing exists."
                    }
                }
                div { class: "favorites-grid",
                    for favorite in favorites.iter() {
                        FavoriteCard { favorite: favorite.clone() }
                    }
                }
            }
        }
    }
}