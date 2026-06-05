use dioxus::prelude::*;

use crate::{
    services::{arena_map_icon, sparkline_end_y, sparkline_points},
    stores::use_insight_panel_store,
    types::{CharacterProfileData, PvpMapStat},
};

use super::common::MetricCard;

#[component]
pub(crate) fn PvpTab(profile: CharacterProfileData) -> Element {
    let rating_text = if profile.pvp_rating > 0 {
        profile.pvp_rating.to_string()
    } else {
        String::from("Unranked")
    };
    let rating_sublabel = if profile.pvp_rating > 0 {
        String::from("Best available rated bracket")
    } else {
        String::from("No public rated bracket found")
    };

    rsx! {
        div { class: "tab-stack",
            div { class: "metrics-grid",
                MetricCard {
                    label: "Rated",
                    value: rating_text,
                    sublabel: rating_sublabel,
                }
                MetricCard {
                    label: "Win rate",
                    value: format!("{}%", profile.arena_win_rate),
                    sublabel: String::from("Across live PvP records"),
                }
                MetricCard {
                    label: "Honor level",
                    value: profile.honor_level.to_string(),
                    sublabel: String::from("Season-long PvP progression"),
                }
                MetricCard {
                    label: "Honorable kills",
                    value: profile.honorable_kills.to_string(),
                    sublabel: String::from("Recorded by the Blizzard profile"),
                }
            }
            div { class: "content-grid two-up",
                section { class: "card-elevated",
                    div { class: "section-heading compact",
                        h2 { "PvP map record" }
                        p { class: "text-muted",
                            "The current Blizzard profile exposes map statistics, so the PvP table now shows live map performance instead of invented match history."
                        }
                    }
                    div { class: "table-panel",
                        if profile.pvp_map_stats.is_empty() {
                            div { class: "empty-state text-muted",
                                "No public PvP map statistics were returned for this character."
                            }
                        } else {
                            for stat in profile.pvp_map_stats.iter() {
                                PvpMapRow { stat: stat.clone() }
                            }
                        }
                    }
                    div { class: "table-actions",
                        button {
                            class: "secondary-link table-cta",
                            r#type: "button",
                            "View Blizzard profile"
                        }
                    }
                }
                InsightPanel {}
            }
        }
    }
}

#[component]
fn InsightPanel() -> Element {
    let panel = use_insight_panel_store(0, true);
    let mut active_tab = panel.active_tab;
    let mut collapsed = panel.collapsed;
    let mut pinned = panel.pinned;

    rsx! {
        section { class: if pinned() { "card-surface note-card strategic-card pinned" } else { "card-surface note-card strategic-card" },
            div { class: "section-heading compact",
                div { class: "note-panel-head",
                    div { class: "note-tabs",
                        button {
                            class: if active_tab() == 0 { "note-chip active" } else { "note-chip" },
                            r#type: "button",
                            onclick: move |_| active_tab.set(0),
                            "Tactical"
                        }
                        button {
                            class: if active_tab() == 1 { "note-chip active" } else { "note-chip" },
                            r#type: "button",
                            onclick: move |_| active_tab.set(1),
                            "Builds"
                        }
                        button {
                            class: if active_tab() == 2 { "note-chip active" } else { "note-chip" },
                            r#type: "button",
                            onclick: move |_| active_tab.set(2),
                            "Cooldowns"
                        }
                    }
                    div { class: "note-toolbar",
                        button {
                            class: if pinned() { "note-tool active" } else { "note-tool" },
                            r#type: "button",
                            onclick: move |_| pinned.set(!pinned()),
                            i { class: "ti ti-pin" }
                            span {
                                if pinned() {
                                    "Pinned"
                                } else {
                                    "Pin"
                                }
                            }
                        }
                        button {
                            class: "note-tool",
                            r#type: "button",
                            onclick: move |_| collapsed.set(!collapsed()),
                            i { class: if collapsed() { "ti ti-chevron-down" } else { "ti ti-chevron-up" } }
                            span {
                                if collapsed() {
                                    "Expand"
                                } else {
                                    "Collapse"
                                }
                            }
                        }
                    }
                }
                h2 { "Queue insights" }
                p { class: "text-muted",
                    "The right rail now behaves more like an active strategy panel, with tabs and quick controls instead of a static note stack."
                }
            }
            if !collapsed() {
                div { class: "insight-list",
                    if active_tab() == 0 {
                        InsightRow {
                            icon: "ti ti-swords",
                            title: "Map pressure",
                            body: "Use the live map record on the left to spot which battlegrounds and arenas are currently trending well.",
                        }
                        InsightRow {
                            icon: "ti ti-shield-half",
                            title: "Defensive cadence",
                            body: "Anchor defensive planning to the maps with the roughest public record instead of assuming a fixed queue pattern.",
                        }
                        InsightRow {
                            icon: "ti ti-flame",
                            title: "Queue read",
                            body: "Treat the public map spread as the real signal for where the next PvP push should focus.",
                        }
                    }
                    if active_tab() == 1 {
                        InsightRow {
                            icon: "ti ti-hammer",
                            title: "Loadout review",
                            body: "Revisit the current build when the live record shows a few maps falling behind the rest of the rotation.",
                        }
                        InsightRow {
                            icon: "ti ti-stars",
                            title: "Talent snapshot",
                            body: "Use live results as the trigger for build changes instead of keeping a prototype recommendation pinned in place.",
                        }
                        InsightRow {
                            icon: "ti ti-arrow-big-up-lines",
                            title: "Upgrade focus",
                            body: "Pair the real PvP record with the live gear inspector when deciding which survivability slots still need work.",
                        }
                    }
                    if active_tab() == 2 {
                        InsightRow {
                            icon: "ti ti-bolt",
                            title: "First trade",
                            body: "Use the toughest live maps to decide which cooldowns must be committed early in a round.",
                        }
                        InsightRow {
                            icon: "ti ti-clock-hour-4",
                            title: "Cycle timing",
                            body: "Let the live queue record drive cooldown timing adjustments instead of a fixed prototype script.",
                        }
                        InsightRow {
                            icon: "ti ti-flag-3",
                            title: "Round close",
                            body: "Protect closing tools for the maps where the public record still shows the widest gap.",
                        }
                    }
                }
            } else {
                div { class: "insight-collapsed text-muted",
                    "Insights collapsed for a tighter right rail."
                }
            }
        }
    }
}

#[component]
pub(crate) fn InsightRow(
    icon: &'static str,
    title: &'static str,
    body: &'static str,
) -> Element {
    rsx! {
        div { class: "insight-row",
            div { class: "insight-icon",
                i { class: "{icon}" }
            }
            div { class: "insight-copy",
                strong { class: "insight-title", "{title}" }
                p { class: "text-muted", "{body}" }
            }
        }
    }
}

#[component]
pub(crate) fn PvpMapRow(stat: PvpMapStat) -> Element {
    let net_wins = stat.won as i16 - stat.lost as i16;
    let result_class = if stat.won >= stat.lost {
        "result-win"
    } else {
        "result-loss"
    };
    let result_text = if stat.won > stat.lost {
        "Favored"
    } else if stat.won == stat.lost {
        "Even"
    } else {
        "Rough"
    };
    let delta_text = format!("{:+}", net_wins);
    let map_icon = arena_map_icon(&stat.map);
    let record_text = format!("{}-{} W-L", stat.won, stat.lost);
    let played_text = format!("{} played", stat.played);

    rsx! {
        div { class: "table-row match-row",
            span { class: "result-badge {result_class}", "{result_text}" }
            div { class: "match-map",
                i { class: "{map_icon}" }
                span { "{stat.map}" }
            }
            span { class: "text-muted", "{record_text}" }
            span { class: "{result_class} mono", "{delta_text}" }
            MatchSparkline { delta: net_wins }
            span { class: "text-muted mono", "{played_text}" }
        }
    }
}

#[component]
fn MatchSparkline(delta: i16) -> Element {
    let points = sparkline_points(delta);
    let tone = if delta >= 0 {
        "var(--color-win)"
    } else {
        "var(--color-loss)"
    };
    let end_y = sparkline_end_y(delta);

    rsx! {
        svg {
            class: "match-sparkline",
            width: "46",
            height: "22",
            view_box: "0 0 46 22",
            polyline {
                points: "{points}",
                fill: "none",
                stroke: "{tone}",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
            }
            circle {
                cx: "42",
                cy: "{end_y}",
                r: "2.5",
                fill: "{tone}",
            }
        }
    }
}