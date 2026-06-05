use dioxus::prelude::*;

use crate::{
    components::{AppShell, FavoriteCard, FeatureCard, MetricCard},
    stores::use_home_view_store,
    types::NavSection,
    Route,
};

#[component]
pub(crate) fn Home() -> Element {
    let store = use_home_view_store();
    let demo = store.demo_profile_link.clone();
    let favorites = (store.favorites)();

    rsx! {
        AppShell {
            current: NavSection::Home,
            profile_link: demo.clone(),
            accent: "#C69B3A",
            accent_muted: "rgba(198,155,58,0.15)",
            section { class: "home-hero card-elevated",
                div { class: "hero-copy",
                    span { class: "eyebrow", "Dark, game-native analytics" }
                    h1 { class: "hero-title",
                        "Desktop companion for PvP, raids, Mythic+, and collection tracking."
                    }
                    p { class: "hero-text",
                        "Live character routes now drive the main competitive and collection surfaces while keeping the compact desktop dashboard feel."
                    }
                    div { class: "hero-actions",
                        Link { class: "primary-link", to: demo.clone(), "Explore the sample character" }
                        Link { class: "secondary-link", to: Route::Favorites {},
                            "Browse saved profiles"
                        }
                    }
                }
                div { class: "hero-preview",
                    div { class: "card-surface preview-card",
                        span { class: "preview-label", "Scout target" }
                        strong { class: "preview-value mono", "Bvb / Ravencrest / EU" }
                        p { class: "text-muted",
                            "Live-backed profile route with compact metrics and class-reactive accents."
                        }
                    }
                    div { class: "preview-metrics",
                        MetricCard {
                            label: "ilvl",
                            value: String::from("528"),
                            sublabel: String::from("Myth track build"),
                        }
                        MetricCard {
                            label: "M+",
                            value: String::from("3,247"),
                            sublabel: String::from("Timed 8/8 dungeons"),
                        }
                        MetricCard {
                            label: "PvP",
                            value: String::from("2,481"),
                            sublabel: String::from("Gladiator push"),
                        }
                        MetricCard {
                            label: "Parse",
                            value: String::from("89%"),
                            sublabel: String::from("Heroic raid average"),
                        }
                    }
                }
            }
            section { class: "feature-grid",
                FeatureCard {
                    icon: "ti ti-swords",
                    title: "PvP pulse",
                    body: "Recent arena sessions, bracket deltas, and queue-ready summaries in a single scan.",
                }
                FeatureCard {
                    icon: "ti ti-flame",
                    title: "Raid performance",
                    body: "Boss parses render as compact colored bars so strong and weak pulls stand out instantly.",
                }
                FeatureCard {
                    icon: "ti ti-door-enter",
                    title: "Mythic+ pace",
                    body: "A score ring anchors the tab while key results and timing status stay readable at a glance.",
                }
                FeatureCard {
                    icon: "ti ti-medal",
                    title: "Collection depth",
                    body: "Mount, pet, title, and achievement progress sits beside the main competitive surfaces.",
                }
            }
            section { class: "favorites-strip card-surface",
                div { class: "section-heading",
                    h2 { "Saved character looks" }
                    p { class: "text-muted", "Pinned demo profiles for the desktop prototype." }
                }
                div { class: "favorites-grid compact",
                    for favorite in favorites.iter() {
                        FavoriteCard { favorite: favorite.clone() }
                    }
                }
            }
        }
    }
}
