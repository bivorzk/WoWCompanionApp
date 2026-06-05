use dioxus::prelude::*;

use crate::{
    services::{metric_icon, metric_trend, theme_for_class},
    types::{Faction, FavoriteCharacter},
};

#[component]
pub(crate) fn MetricCard(label: &'static str, value: String, sublabel: String) -> Element {
    let icon = metric_icon(label);
    let (trend, trend_class) = metric_trend(label);
    let trend_marker = match trend_class {
        "up" => "↑",
        "down" => "↓",
        _ => "•",
    };

    rsx! {
        div { class: "card-surface metric-card",
            div { class: "metric-card-head",
                div { class: "metric-icon",
                    i { class: "{icon}" }
                }
                div { class: "metric-trend {trend_class}",
                    span { class: "metric-trend-marker mono", "{trend_marker}" }
                    span { "{trend}" }
                }
            }
            span { class: "metric-label", "{label}" }
            div { class: "metric-value-row",
                span { class: "metric-value mono", "{value}" }
                span { class: "metric-direction {trend_class}", "{trend_marker}" }
            }
            span { class: "metric-sub", "{sublabel}" }
        }
    }
}

#[component]
pub(crate) fn FeatureCard(icon: &'static str, title: &'static str, body: &'static str) -> Element {
    rsx! {
        article { class: "card-surface feature-card",
            div { class: "feature-icon",
                i { class: "{icon}" }
            }
            h2 { class: "feature-title", "{title}" }
            p { class: "text-muted", "{body}" }
        }
    }
}

#[component]
pub(crate) fn FactionBadge(faction: Faction) -> Element {
    rsx! {
        span { class: "faction-badge {faction.class_name()}",
            FactionIcon { faction }
            span { "{faction.label()}" }
        }
    }
}

#[component]
fn FactionMark(faction: Faction) -> Element {
    rsx! {
        span { class: "faction-mark {faction.class_name()}",
            FactionIcon { faction }
        }
    }
}

#[component]
fn FactionIcon(faction: Faction) -> Element {
    match faction {
        Faction::Alliance => rsx! {
            svg {
                class: "faction-icon",
                width: "16",
                height: "16",
                view_box: "0 0 24 24",
                path {
                    d: "M12 2 5.5 5.2v6.1c0 4.6 2.9 8.3 6.5 10.7 3.6-2.4 6.5-6.1 6.5-10.7V5.2L12 2Z",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.8",
                    stroke_linejoin: "round",
                }
                path {
                    d: "m12 6.7 1.45 2.95 3.26.47-2.35 2.29.55 3.24L12 14.1l-2.91 1.55.56-3.24-2.36-2.29 3.26-.47L12 6.7Z",
                    fill: "currentColor",
                }
            }
        },
        Faction::Horde => rsx! {
            svg {
                class: "faction-icon",
                width: "16",
                height: "16",
                view_box: "0 0 24 24",
                path {
                    d: "M12 2.5 16.8 7l-2 3.1 3.6 2.1-3.1 9.3-3.3-3.6-3.3 3.6-3.1-9.3 3.6-2.1-2-3.1L12 2.5Z",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.8",
                    stroke_linejoin: "round",
                }
                path {
                    d: "M12 7.4 14.6 10l-1.05 1.7 2 1.12L13.9 17l-1.9-1.88L10.1 17l-1.65-4.18 2-1.11L9.4 10 12 7.4Z",
                    fill: "currentColor",
                }
            }
        },
    }
}

#[component]
pub(crate) fn FavoriteCard(favorite: FavoriteCharacter) -> Element {
    let theme = theme_for_class(favorite.class_name);

    rsx! {
        Link {
            class: "favorite-card card-surface",
            to: favorite.route,
            style: "--favorite-accent: {theme.accent}; --favorite-muted: {theme.accent_muted};",
            div { class: "favorite-card-head",
                div { class: "favorite-meta",
                    FactionMark { faction: favorite.faction }
                    span { class: "favorite-class", "{favorite.spec} {favorite.class_name}" }
                }
                span { class: "favorite-region mono", "{favorite.region}" }
            }
            strong { class: "favorite-name", "{favorite.name}" }
            span { class: "text-muted", "{favorite.realm}" }
            div { class: "favorite-stats",
                span { class: "mono", "ilvl {favorite.item_level}" }
                span { class: "mono", "M+ {favorite.mythic_score}" }
                span { class: "mono", "PvP {favorite.pvp_rating}" }
            }
        }
    }
}

#[component]
pub(crate) fn SettingCard(title: &'static str, description: &'static str, enabled: bool) -> Element {
    rsx! {
        article { class: "card-surface setting-card",
            div { class: "setting-head",
                h2 { class: "feature-title", "{title}" }
                span { class: if enabled { "toggle-pill on" } else { "toggle-pill" },
                    if enabled {
                        "On"
                    } else {
                        "Off"
                    }
                }
            }
            p { class: "text-muted", "{description}" }
        }
    }
}

#[component]
pub(crate) fn AccentChip(label: &'static str, accent: &'static str, muted: &'static str) -> Element {
    rsx! {
        div {
            class: "accent-chip",
            style: "--sample-accent: {accent}; --sample-muted: {muted};",
            span { class: "accent-swatch" }
            span { "{label}" }
        }
    }
}