use dioxus::prelude::*;

use crate::{types::{CharacterProfileData, OverallSection}, Route};

use super::{
    common::{FactionBadge, MetricCard},
    progression::{MythicRunRow, RaidDifficultyRow, ScoreRing},
    pvp::PvpMapRow,
};

#[component]
pub(crate) fn OverallSectionButton(
    label: &'static str,
    icon: &'static str,
    section: OverallSection,
    selected: Signal<OverallSection>,
) -> Element {
    let is_active = selected() == section;

    rsx! {
        button {
            class: if is_active { "overall-switch-btn active" } else { "overall-switch-btn" },
            r#type: "button",
            onclick: move |_| selected.set(section),
            i { class: "{icon}" }
            span { "{label}" }
        }
    }
}

#[component]
pub(crate) fn OverallRaidsPanel(profile: CharacterProfileData, profile_link: Route) -> Element {
    rsx! {
        div { class: "overall-content",
            div { class: "metrics-grid",
                MetricCard {
                    label: "Progress",
                    value: profile.raid_progress.clone(),
                    sublabel: String::from("Current heroic clear"),
                }
                MetricCard {
                    label: "Tier",
                    value: if profile.raid_tier.is_empty() { String::from("Unknown") } else { profile.raid_tier.clone() },
                    sublabel: String::from("Current Raider.IO tier"),
                }
                MetricCard {
                    label: "Modes",
                    value: profile.raid_difficulties.len().to_string(),
                    sublabel: String::from("Difficulties with public data"),
                }
                MetricCard {
                    label: "Faction",
                    value: String::from(profile.faction.label()),
                    sublabel: String::from("Current featured roster side"),
                }
            }
            div { class: "content-grid two-up overall-grid",
                section { class: "card-elevated",
                    div { class: "section-heading compact",
                        h2 { "Raid snapshot" }
                        p { class: "text-muted",
                            "The overall raid panel now reflects public progression directly instead of seeded parse rows."
                        }
                    }
                    div { class: "parse-panel",
                        if profile.raid_difficulties.is_empty() {
                            div { class: "empty-state text-muted",
                                "No public raid difficulty progress was returned for this character."
                            }
                        } else {
                            for progress in profile.raid_difficulties.iter().take(4) {
                                RaidDifficultyRow { progress: progress.clone() }
                            }
                        }
                    }
                }
                section { class: "card-surface overall-note-card",
                    div { class: "section-heading compact",
                        h2 { "Featured raider" }
                        p { class: "text-muted",
                            "The featured raider keeps raid performance, dungeon pace, and PvP context visible without leaving the top-level dashboard."
                        }
                    }
                    div { class: "overall-featured-player",
                        FactionBadge { faction: profile.faction }
                        div {
                            strong { class: "feature-title", "{profile.name}" }
                            p { class: "text-muted",
                                "{profile.spec} {profile.class_name} - {profile.hero_talent}"
                            }
                        }
                    }
                    Link { class: "primary-link", to: profile_link, "Open character profile" }
                }
            }
        }
    }
}

#[component]
pub(crate) fn OverallMythicPanel(profile: CharacterProfileData, profile_link: Route) -> Element {
    let timed_keys = timed_key_count(&profile.mythic_runs);
    let best_route = best_route_name(&profile.mythic_runs);

    rsx! {
        div { class: "overall-content",
            div { class: "content-grid mythic-grid overall-grid",
                section { class: "card-surface score-ring-card",
                    ScoreRing { score: profile.mythic_score, season: "TWW S4" }
                    div { class: "score-ring-copy",
                        h2 { "Mythic+ overview" }
                        p { class: "text-muted",
                            "Select Mythic+ to see current routing form, season score, and the most recent key timings."
                        }
                    }
                }
                div { class: "metrics-grid compact-metrics",
                    MetricCard {
                        label: "Score",
                        value: profile.mythic_score.to_string(),
                        sublabel: String::from("Season aggregate"),
                    }
                    MetricCard {
                        label: "Recent keys",
                        value: profile.weekly_keys.to_string(),
                        sublabel: String::from("Recent public runs visible in Raider.IO"),
                    }
                    MetricCard {
                        label: "Timed pool",
                        value: format!("{timed_keys}/{}", profile.mythic_runs.len()),
                        sublabel: String::from("Timed public runs"),
                    }
                    MetricCard {
                        label: "Best route",
                        value: best_route,
                        sublabel: String::from("Highest exposed key route"),
                    }
                }
            }
            section { class: "card-elevated",
                div { class: "section-heading compact",
                    h2 { "Recent public key log" }
                    p { class: "text-muted",
                        "Mythic+ runs stay table-like and compact so route quality is readable without feeling spreadsheet-heavy."
                    }
                }
                div { class: "table-panel",
                    for run in profile.mythic_runs.iter().take(4) {
                        MythicRunRow { run: run.clone() }
                    }
                }
                div { class: "overall-link-row",
                    Link { class: "secondary-link", to: profile_link, "View full character breakdown" }
                }
            }
        }
    }
}

#[component]
pub(crate) fn OverallPvpPanel(profile: CharacterProfileData, profile_link: Route) -> Element {
    let rating_text = if profile.pvp_rating > 0 {
        profile.pvp_rating.to_string()
    } else {
        String::from("Unranked")
    };

    rsx! {
        div { class: "overall-content",
            div { class: "metrics-grid",
                MetricCard {
                    label: "Rated",
                    value: rating_text,
                    sublabel: String::from("Best available rated bracket"),
                }
                MetricCard {
                    label: "Win rate",
                    value: format!("{}%", profile.arena_win_rate),
                    sublabel: String::from("Across live PvP records"),
                }
                MetricCard {
                    label: "Honor",
                    value: profile.honor_level.to_string(),
                    sublabel: String::from("Season-long progress"),
                }
                MetricCard {
                    label: "Honorable kills",
                    value: profile.honorable_kills.to_string(),
                    sublabel: String::from("Recorded by Blizzard"),
                }
            }
            div { class: "content-grid two-up overall-grid",
                section { class: "card-elevated",
                    div { class: "section-heading compact",
                        h2 { "PvP map record" }
                        p { class: "text-muted",
                            "The overall PvP panel now mirrors live map statistics instead of seeded match history."
                        }
                    }
                    div { class: "table-panel",
                        if profile.pvp_map_stats.is_empty() {
                            div { class: "empty-state text-muted",
                                "No public PvP map statistics were returned for this character."
                            }
                        } else {
                            for stat in profile.pvp_map_stats.iter().take(4) {
                                PvpMapRow { stat: stat.clone() }
                            }
                        }
                    }
                }
                section { class: "card-surface overall-note-card",
                    div { class: "section-heading compact",
                        h2 { "Current push" }
                        p { class: "text-muted",
                            "The featured profile is trending upward through fast rounds and clean cooldown rotations into caster-heavy lobbies."
                        }
                    }
                    div { class: "overall-featured-player",
                        FactionBadge { faction: profile.faction }
                        div {
                            strong { class: "feature-title", "{profile.name}" }
                            p { class: "text-muted",
                                "{profile.spec} {profile.class_name} - {profile.arena_win_rate}% round win rate"
                            }
                        }
                    }
                    Link { class: "primary-link", to: profile_link, "Inspect PvP profile" }
                }
            }
        }
    }
}

fn timed_key_count(runs: &[crate::types::MythicRun]) -> usize {
    runs.iter().filter(|run| run.result == "Timed").count()
}

fn best_route_name(runs: &[crate::types::MythicRun]) -> String {
    runs.iter()
        .max_by_key(|run| run.level)
        .map(|run| run.dungeon.clone())
        .unwrap_or_else(|| String::from("No runs"))
}