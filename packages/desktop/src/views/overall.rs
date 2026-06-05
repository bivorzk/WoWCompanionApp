use dioxus::prelude::*;

use crate::{
    components::{
        AppShell, FactionBadge, OverallMythicPanel, OverallPvpPanel, OverallRaidsPanel,
        OverallSectionButton,
    },
    services::theme_for_class,
    stores::use_overall_view_store,
    types::{NavSection, OverallSection},
};

#[component]
pub(crate) fn Overall() -> Element {
    let store = use_overall_view_store();
    let demo_route = store.profile_link.clone();
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
            current: NavSection::Overall,
            profile_link: demo_route.clone(),
            accent: theme.accent,
            accent_muted: theme.accent_muted,
            if let Some(profile) = profile {
                section { class: "overall-root",
                    section { class: "overall-header card-elevated",
                        div { class: "overall-header-copy",
                            span { class: "eyebrow", "Overall section" }
                            h1 { class: "section-title", "Switch between raids, Mythic+, and PvP." }
                            p { class: "text-muted",
                                "This screen gives the featured character a mode-specific summary without leaving the high-level dashboard shell."
                            }
                        }
                        div { class: "overall-spotlight card-surface",
                            div { class: "overall-profile-line",
                                FactionBadge { faction: profile.faction }
                                strong { class: "overall-profile-name", "{profile.name}" }
                            }
                            div { class: "identity-row",
                                span { class: "class-badge", "{profile.spec} {profile.class_name}" }
                                span { class: "char-realm", "{profile.realm} / {profile.region}" }
                                span { class: "char-realm", "{profile.guild}" }
                            }
                        }
                    }
                    div { class: "overall-section-switch",
                        OverallSectionButton {
                            label: "Raids",
                            icon: "ti ti-flame",
                            section: OverallSection::Raids,
                            selected: store.selected_section,
                        }
                        OverallSectionButton {
                            label: "Mythic+",
                            icon: "ti ti-door-enter",
                            section: OverallSection::Mythic,
                            selected: store.selected_section,
                        }
                        OverallSectionButton {
                            label: "PvP",
                            icon: "ti ti-swords",
                            section: OverallSection::Pvp,
                            selected: store.selected_section,
                        }
                    }
                    div { class: "overall-panel",
                        if (store.selected_section)() == OverallSection::Raids {
                            OverallRaidsPanel {
                                profile: profile.clone(),
                                profile_link: demo_route.clone(),
                            }
                        }
                        if (store.selected_section)() == OverallSection::Mythic {
                            OverallMythicPanel {
                                profile: profile.clone(),
                                profile_link: demo_route.clone(),
                            }
                        }
                        if (store.selected_section)() == OverallSection::Pvp {
                            OverallPvpPanel { profile, profile_link: demo_route }
                        }
                    }
                }
            } else if let Some(error) = error {
                section { class: "section-card card-elevated",
                    div { class: "section-heading",
                        h1 { class: "section-title", "Live Overview Unavailable" }
                        p { class: "text-muted",
                            "The overall route is now backed by live APIs, but the featured profile request failed."
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
                        h1 { class: "section-title", "Loading Live Overview" }
                        p { class: "text-muted",
                            "Fetching the featured character before building the overall dashboard sections."
                        }
                    }
                }
            }
        }
    }
}