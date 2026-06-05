use dioxus::prelude::*;

use crate::{
    stores::use_tab_panel_store,
    types::{AchievementEntry, CharacterProfileData, CollectionAvailability, CollectionItem},
};

use super::common::MetricCard;

#[component]
pub(crate) fn CollectionTab(profile: CharacterProfileData) -> Element {
    let active_tab = use_tab_panel_store(0).active_tab;
    let visible_items = match active_tab() {
        0 => profile.mount_items.clone(),
        1 => profile.pet_items.clone(),
        _ => Vec::new(),
    };
    let visible_total = visible_items.len();
    let shown_badge = match active_tab() {
        0 => collection_shown_badge(visible_total, profile.mounts_availability),
        1 => collection_shown_badge(visible_total, profile.pets_availability),
        _ => String::new(),
    };
    let total_badge = match active_tab() {
        0 => collection_total_badge(profile.mounts_collected, "mounts", profile.mounts_availability),
        1 => collection_total_badge(profile.pets_collected, "pets", profile.pets_availability),
        _ => String::new(),
    };
    let empty_state = match active_tab() {
        0 => collection_empty_state("mount", profile.mounts_availability),
        1 => collection_empty_state("pet", profile.pets_availability),
        _ => String::from("No recent achievement events were returned for this character."),
    };
    let (board_title, board_copy, board_action) = match active_tab() {
        0 => (
            "Mount highlights",
            collection_board_copy("mount", profile.mounts_availability),
            "Open mount collection",
        ),
        1 => (
            "Pet highlights",
            collection_board_copy("pet", profile.pets_availability),
            "Open pet journal",
        ),
        _ => (
            "Achievement highlights",
            String::from(
                "The achievement segment now reflects recent live achievement events instead of reusing the collection tiles.",
            ),
            "View recent achievements",
        ),
    };
    let mount_metric_value = collection_metric_value(profile.mounts_collected, profile.mounts_availability);
    let pet_metric_value = collection_metric_value(profile.pets_collected, profile.pets_availability);
    let mount_metric_sublabel = collection_metric_sublabel(
        "Collected across account",
        profile.mounts_availability,
    );
    let pet_metric_sublabel = collection_metric_sublabel(
        "Battle pet stable",
        profile.pets_availability,
    );

    rsx! {
        div { class: "tab-stack",
            div { class: "metrics-grid",
                MetricCard {
                    label: "Mounts",
                    value: mount_metric_value,
                    sublabel: mount_metric_sublabel,
                }
                MetricCard {
                    label: "Pets",
                    value: pet_metric_value,
                    sublabel: pet_metric_sublabel,
                }
                MetricCard {
                    label: "Achievements",
                    value: profile.achievements_unlocked.to_string(),
                    sublabel: String::from("Unlocked on this character"),
                }
                MetricCard {
                    label: "Points",
                    value: profile.achievement_points.to_string(),
                    sublabel: String::from("Achievement total"),
                }
            }
            div { class: "content-grid two-up",
                section { class: "card-elevated collection-board",
                    div { class: "section-heading compact",
                        h2 { "{board_title}" }
                        p { class: "text-muted", "{board_copy}" }
                    }
                    if active_tab() != 2 {
                        div { class: "panel-badge-row",
                            span { class: "status-chip accent", "{shown_badge}" }
                            span { class: "status-chip", "{total_badge}" }
                        }
                    }
                    if active_tab() == 2 {
                        div { class: "achievement-feed",
                            if profile.achievements.is_empty() {
                                div { class: "empty-state text-muted",
                                    "No recent achievement events were returned for this character."
                                }
                            } else {
                                for achievement in profile.achievements.iter().take(4) {
                                    AchievementRow { achievement: achievement.clone() }
                                }
                            }
                        }
                    } else {
                        div { class: "collect-grid",
                            if visible_items.is_empty() {
                                div { class: "empty-state text-muted", "{empty_state}" }
                            } else {
                                for item in visible_items.iter() {
                                    CollectTile { item: item.clone() }
                                }
                            }
                        }
                    }
                    div { class: "table-actions",
                        button {
                            class: "secondary-link table-cta",
                            r#type: "button",
                            "{board_action}"
                        }
                    }
                }
                CollectionFocusPanel { profile: profile.clone(), active_tab }
            }
            section { class: "card-elevated achievement-panel",
                div { class: "section-heading compact",
                    h2 { "Achievement feed" }
                    p { class: "text-muted",
                        "The feed now stands on its own so recent wins feel like a timeline instead of a squeezed secondary card."
                    }
                }
                div { class: "achievement-feed",
                    if profile.achievements.is_empty() {
                        div { class: "empty-state text-muted",
                            "No recent achievement events were returned for this character."
                        }
                    } else {
                        for achievement in profile.achievements.iter() {
                            AchievementRow { achievement: achievement.clone() }
                        }
                    }
                }
                div { class: "table-actions",
                    button { class: "secondary-link table-cta", r#type: "button",
                        "View all achievements"
                    }
                }
            }
        }
    }
}

#[component]
fn CollectionFocusPanel(profile: CharacterProfileData, active_tab: Signal<usize>) -> Element {
    let mount_item = profile.mount_items.first();
    let pet_item = profile.pet_items.first();
    let recent_achievement = profile.achievements.first();
    let mount_status_badge = collection_feature_status(
        profile.mounts_collected,
        "mounts",
        profile.mounts_availability,
    );
    let pet_status_badge = collection_feature_status(
        profile.pets_collected,
        "pets",
        profile.pets_availability,
    );

    let (feature_label, feature_title, feature_source, feature_status) = match active_tab() {
        0 => (
            String::from("Featured mount"),
            collection_feature_title(
                mount_item,
                "No mounts loaded",
                profile.mounts_availability,
            ),
            collection_feature_source("mount", profile.mounts_availability),
            collection_feature_status(
                profile.mounts_collected,
                "mounts",
                profile.mounts_availability,
            ),
        ),
        1 => (
            String::from("Featured pet"),
            collection_feature_title(
                pet_item,
                "No pets loaded",
                profile.pets_availability,
            ),
            collection_feature_source("pet", profile.pets_availability),
            collection_feature_status(
                profile.pets_collected,
                "pets",
                profile.pets_availability,
            ),
        ),
        _ => (
            String::from("Recent achievement"),
            recent_achievement
                .map(|item| item.name.clone())
                .unwrap_or_else(|| String::from("No recent achievements")),
            String::from("Recent events from the Blizzard achievement feed."),
            format!("{} achievements", profile.achievements_unlocked),
        ),
    };

    rsx! {
        section { class: "card-surface tab-side-panel collection-side-panel",
            div { class: "section-heading compact",
                div { class: "note-tabs",
                    button {
                        class: if active_tab() == 0 { "note-chip active" } else { "note-chip" },
                        r#type: "button",
                        onclick: move |_| active_tab.set(0),
                        "Mounts"
                    }
                    button {
                        class: if active_tab() == 1 { "note-chip active" } else { "note-chip" },
                        r#type: "button",
                        onclick: move |_| active_tab.set(1),
                        "Pets"
                    }
                    button {
                        class: if active_tab() == 2 { "note-chip active" } else { "note-chip" },
                        r#type: "button",
                        onclick: move |_| active_tab.set(2),
                        "Achievements"
                    }
                }
                h2 { "Collector board" }
                p { class: "text-muted",
                    "Collection keeps public Blizzard availability explicit, so hidden or missing mount and pet data no longer reads like zero owned."
                }
            }
            div { class: "panel-badge-row",
                span { class: "status-chip accent", "{mount_status_badge}" }
                span { class: "status-chip", "{pet_status_badge}" }
                span { class: "status-chip", "{profile.achievements_unlocked} achievements" }
            }
            div { class: "collection-feature card-elevated",
                span { class: "metric-label", "{feature_label}" }
                strong { class: "feature-title", "{feature_title}" }
                p { class: "text-muted", "{feature_source}" }
                span { class: "status-chip accent", "{feature_status}" }
            }
            div { class: "insight-list compact-insights",
                if active_tab() == 0 {
                    CollectionInsightRow {
                        icon: "ti ti-bird",
                        title: "Live mount view",
                        body: collection_insight_body("mount", profile.mounts_availability),
                    }
                    CollectionInsightRow {
                        icon: "ti ti-map-pin",
                        title: "Mount count",
                        body: collection_count_insight_body("mounts", profile.mounts_availability),
                    }
                }
                if active_tab() == 1 {
                    CollectionInsightRow {
                        icon: "ti ti-paw",
                        title: "Live pet view",
                        body: collection_insight_body("pet", profile.pets_availability),
                    }
                    CollectionInsightRow {
                        icon: "ti ti-medal",
                        title: "Pet count",
                        body: collection_count_insight_body("pets", profile.pets_availability),
                    }
                }
                if active_tab() == 2 {
                    CollectionInsightRow {
                        icon: "ti ti-stars",
                        title: "Achievement feed",
                        body: String::from(
                            "Recent achievement events below now come from the Blizzard character achievements endpoint.",
                        ),
                    }
                    CollectionInsightRow {
                        icon: "ti ti-trophy",
                        title: "Achievement total",
                        body: String::from(
                            "The achievements metric reflects the live total quantity returned for this character.",
                        ),
                    }
                }
            }
        }
    }
}

#[component]
fn CollectionInsightRow(icon: &'static str, title: &'static str, body: String) -> Element {
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

fn collection_metric_value(total: u16, availability: CollectionAvailability) -> String {
    match availability {
        CollectionAvailability::Available => total.to_string(),
        CollectionAvailability::Restricted => String::from("Private"),
        CollectionAvailability::Unavailable => String::from("N/A"),
    }
}

fn collection_metric_sublabel(default_label: &str, availability: CollectionAvailability) -> String {
    match availability {
        CollectionAvailability::Available => String::from(default_label),
        CollectionAvailability::Restricted => {
            String::from("Not exposed on this public Blizzard profile")
        }
        CollectionAvailability::Unavailable => String::from("Collection data unavailable right now"),
    }
}

fn collection_shown_badge(visible_total: usize, availability: CollectionAvailability) -> String {
    match availability {
        CollectionAvailability::Available => format!("{visible_total} shown"),
        CollectionAvailability::Restricted => String::from("Profile restricted"),
        CollectionAvailability::Unavailable => String::from("Collection unavailable"),
    }
}

fn collection_total_badge(
    total: u16,
    plural_label: &str,
    availability: CollectionAvailability,
) -> String {
    match availability {
        CollectionAvailability::Available => format!("{total} collected"),
        CollectionAvailability::Restricted => format!("Public {plural_label} hidden"),
        CollectionAvailability::Unavailable => format!("{plural_label} unavailable"),
    }
}

fn collection_board_copy(kind: &str, availability: CollectionAvailability) -> String {
    match availability {
        CollectionAvailability::Available => format!(
            "The {kind} segment surfaces live Blizzard {kind} entries with their actual names instead of tiny placeholder tiles."
        ),
        CollectionAvailability::Restricted => format!(
            "Blizzard is not exposing the {kind} collection on this public profile, so the app now keeps that state explicit instead of showing a fake zero."
        ),
        CollectionAvailability::Unavailable => format!(
            "The app could not load the {kind} collection right now, so this lane stays explicit instead of implying the character owns none."
        ),
    }
}

fn collection_empty_state(kind: &str, availability: CollectionAvailability) -> String {
    match availability {
        CollectionAvailability::Available => {
            format!("No live {kind} entries were returned for this character.")
        }
        CollectionAvailability::Restricted => format!(
            "Blizzard did not expose the {kind} collection for this character on the public profile."
        ),
        CollectionAvailability::Unavailable => {
            format!("The {kind} collection request did not complete successfully.")
        }
    }
}

fn collection_feature_title(
    item: Option<&CollectionItem>,
    empty_title: &str,
    availability: CollectionAvailability,
) -> String {
    match availability {
        CollectionAvailability::Available => item
            .map(|entry| entry.name.clone())
            .unwrap_or_else(|| String::from(empty_title)),
        CollectionAvailability::Restricted => String::from("Private collection"),
        CollectionAvailability::Unavailable => String::from("Collection unavailable"),
    }
}

fn collection_feature_source(kind: &str, availability: CollectionAvailability) -> String {
    match availability {
        CollectionAvailability::Available => {
            format!("Pulled from the live Blizzard {kind} collection.")
        }
        CollectionAvailability::Restricted => format!(
            "Blizzard did not expose the {kind} collection on the current public profile."
        ),
        CollectionAvailability::Unavailable => {
            format!("The {kind} collection request did not complete successfully.")
        }
    }
}

fn collection_feature_status(
    total: u16,
    plural_label: &str,
    availability: CollectionAvailability,
) -> String {
    match availability {
        CollectionAvailability::Available => format!("{total} {plural_label}"),
        CollectionAvailability::Restricted => String::from("Access restricted"),
        CollectionAvailability::Unavailable => String::from("Not available"),
    }
}

fn collection_insight_body(kind: &str, availability: CollectionAvailability) -> String {
    match availability {
        CollectionAvailability::Available => format!(
            "Highlighted {kind}s now come directly from the live Blizzard collection instead of a seeded wishlist."
        ),
        CollectionAvailability::Restricted => format!(
            "This profile is not exposing its {kind} collection publicly, so the app now treats that as restricted data instead of zero owned."
        ),
        CollectionAvailability::Unavailable => format!(
            "The {kind} collection request failed, so the panel now calls out the missing data instead of inventing an empty collection."
        ),
    }
}

fn collection_count_insight_body(
    plural_label: &str,
    availability: CollectionAvailability,
) -> String {
    match availability {
        CollectionAvailability::Available => {
            format!("Use the live {plural_label} total as the high-level progress signal for this tab.")
        }
        CollectionAvailability::Restricted => format!(
            "The count badges stay explicit when Blizzard does not expose the public {plural_label} total."
        ),
        CollectionAvailability::Unavailable => {
            format!("The count badges now distinguish unavailable {plural_label} data from a real zero.")
        }
    }
}

#[component]
fn CollectTile(item: CollectionItem) -> Element {
    rsx! {
        div {
            class: if item.collected { "collect-item" } else { "collect-item uncollected" },
            title: "{item.name} - {item.source}",
            div { class: "collect-head",
                span { class: "collect-label mono", "{item.label}" }
                span { class: "status-chip collect-state",
                    if item.collected {
                        "Collected"
                    } else {
                        "Missing"
                    }
                }
            }
            strong { class: "collect-name", "{item.name}" }
            span { class: "collect-source text-muted", "{item.source}" }
        }
    }
}

#[component]
fn AchievementRow(achievement: AchievementEntry) -> Element {
    rsx! {
        div { class: "achievement-row",
            div { class: "achievement-icon",
                i { class: "{achievement.icon}" }
            }
            div {
                div { class: "achievement-name", "{achievement.name}" }
                div { class: "achievement-desc", "{achievement.description}" }
            }
            span { class: "achievement-ts mono", "{achievement.timestamp}" }
        }
    }
}
