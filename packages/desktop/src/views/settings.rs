use dioxus::prelude::*;

use crate::{
    components::{AccentChip, AppShell, SettingCard},
    services::CompanionService,
    types::NavSection,
};

#[component]
pub(crate) fn Settings() -> Element {
    let demo = CompanionService::mock().sample_profile_route();

    rsx! {
        AppShell {
            current: NavSection::Settings,
            profile_link: demo,
            accent: "#9896A0",
            accent_muted: "rgba(152,150,160,0.15)",
            section { class: "section-card card-elevated",
                div { class: "section-heading",
                    h1 { class: "section-title", "Interface Settings" }
                    p { class: "text-muted",
                        "These controls are intentionally static. The goal is to establish the shape and tone of the desktop UX without introducing backend or persistence work."
                    }
                }
                div { class: "settings-grid",
                    SettingCard {
                        title: "Class reactive accent",
                        description: "The app shell picks its highlight color from the active character class to keep every profile visually distinct.",
                        enabled: true,
                    }
                    SettingCard {
                        title: "Dense analytics layout",
                        description: "Cards, tables, and compact labels stay readable without drifting into a cluttered MMO addon aesthetic.",
                        enabled: true,
                    }
                    SettingCard {
                        title: "Privacy banners",
                        description: "Reserve space for private-profile or missing-data states before any live API integration lands.",
                        enabled: false,
                    }
                    SettingCard {
                        title: "Compact collection tiles",
                        description: "Collection progress uses low-noise tiles so mounts and achievements support the profile instead of overwhelming it.",
                        enabled: true,
                    }
                }
                div { class: "legend-card card-surface",
                    h2 { "Class accent legend" }
                    div { class: "accent-samples",
                        AccentChip {
                            label: "Paladin",
                            accent: "#F48CBA",
                            muted: "rgba(244,140,186,0.15)",
                        }
                        AccentChip {
                            label: "Druid",
                            accent: "#FF7C0A",
                            muted: "rgba(255,124,10,0.15)",
                        }
                        AccentChip {
                            label: "Mage",
                            accent: "#3FC7EB",
                            muted: "rgba(63,199,235,0.15)",
                        }
                        AccentChip {
                            label: "Warlock",
                            accent: "#8788EE",
                            muted: "rgba(135,136,238,0.15)",
                        }
                    }
                }
            }
        }
    }
}