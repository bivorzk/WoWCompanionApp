use dioxus::prelude::*;

use crate::{
    services::{mythic_dungeon_icon, parse_color},
    stores::use_tab_panel_store,
    types::{CharacterProfileData, MythicRun, RaidDifficultyProgress},
};

use super::{common::MetricCard, pvp::InsightRow};

#[component]
pub(crate) fn RaidsTab(profile: CharacterProfileData) -> Element {
    rsx! {
        div { class: "tab-stack",
            div { class: "metrics-grid",
                MetricCard {
                    label: "Progress",
                    value: profile.raid_progress.clone(),
                    sublabel: String::from("Best public raid clear returned by Raider.IO"),
                }
                MetricCard {
                    label: "Tier",
                    value: if profile.raid_tier.is_empty() { String::from("Unknown") } else { profile.raid_tier.clone() },
                    sublabel: String::from("Current live raid tier label"),
                }
                MetricCard {
                    label: "Modes",
                    value: profile.raid_difficulties.len().to_string(),
                    sublabel: String::from("Difficulties with public progress"),
                }
                MetricCard {
                    label: "Role",
                    value: String::from(profile.role),
                    sublabel: String::from("Current character role"),
                }
            }
            div { class: "content-grid two-up",
                section { class: "card-elevated raid-breakdown-card",
                    div { class: "section-heading compact",
                        h2 { "Public raid clears" }
                        p { class: "text-muted",
                            "Raid difficulty progress now comes from the live Raider.IO clear data instead of the older summary-only payload."
                        }
                    }
                    div { class: "parse-panel",
                        if profile.raid_difficulties.is_empty() {
                            div { class: "empty-state text-muted",
                                "No public raid difficulty progress was returned for this character."
                            }
                        } else {
                            for progress in profile.raid_difficulties.iter() {
                                RaidDifficultyRow { progress: progress.clone() }
                            }
                        }
                    }
                    div { class: "table-actions",
                        button {
                            class: "secondary-link table-cta",
                            r#type: "button",
                            "Open raid summary"
                        }
                    }
                }
                RaidIntelPanel { profile: profile.clone() }
            }
        }
    }
}

#[component]
fn RaidIntelPanel(profile: CharacterProfileData) -> Element {
    let mut active_tab = use_tab_panel_store(0).active_tab;

    rsx! {
        section { class: "card-surface tab-side-panel raid-side-panel",
            div { class: "section-heading compact",
                div { class: "note-tabs",
                    button {
                        class: if active_tab() == 0 { "note-chip active" } else { "note-chip" },
                        r#type: "button",
                        onclick: move |_| active_tab.set(0),
                        "Assignments"
                    }
                    button {
                        class: if active_tab() == 1 { "note-chip active" } else { "note-chip" },
                        r#type: "button",
                        onclick: move |_| active_tab.set(1),
                        "Loot"
                    }
                    button {
                        class: if active_tab() == 2 { "note-chip active" } else { "note-chip" },
                        r#type: "button",
                        onclick: move |_| active_tab.set(2),
                        "Cooldowns"
                    }
                }
                h2 { "Raid command" }
                p { class: "text-muted",
                    "The raid panel stays attached to live public progression so the right rail no longer depends on fake parse statistics."
                }
            }
            div { class: "panel-badge-row",
                span { class: "status-chip accent", "{profile.raid_progress}" }
                span { class: "status-chip",
                    if profile.raid_tier.is_empty() {
                        "Tier unknown"
                    } else {
                        "{profile.raid_tier}"
                    }
                }
                span { class: "status-chip", "{profile.role}" }
            }
            div { class: "insight-list compact-insights",
                if active_tab() == 0 {
                    InsightRow {
                        icon: "ti ti-shield-half",
                        title: "Progress coverage",
                        body: "Use the live raid progression bars on the left to see which difficulties still need the most attention.",
                    }
                    InsightRow {
                        icon: "ti ti-users-group",
                        title: "Roster view",
                        body: "Treat the public raid summary as the source of truth before planning any next-step raid work.",
                    }
                    InsightRow {
                        icon: "ti ti-bolt",
                        title: "Pull priority",
                        body: "The live difficulty breakdown makes it clear which raid mode still has the biggest gap to close.",
                    }
                }
                if active_tab() == 1 {
                    InsightRow {
                        icon: "ti ti-crown",
                        title: "Gear review",
                        body: "Pair the live raid summary with the gear inspector to decide which equipped slots still need attention.",
                    }
                    InsightRow {
                        icon: "ti ti-sparkles",
                        title: "Upgrade source",
                        body: "The raid tab now stays grounded in live public data instead of inventing boss-specific loot recommendations.",
                    }
                    InsightRow {
                        icon: "ti ti-shield",
                        title: "Vault planning",
                        body: "Use the real loadout and current raid progress together when choosing the next weekly upgrade lane.",
                    }
                }
                if active_tab() == 2 {
                    InsightRow {
                        icon: "ti ti-clock-hour-4",
                        title: "Cooldown timing",
                        body: "Anchor raid cooldown planning to the difficulty that still shows the most room to clear.",
                    }
                    InsightRow {
                        icon: "ti ti-heart-handshake",
                        title: "Recovery window",
                        body: "Use the live progression state to focus utility planning where progression is still active rather than on farm content.",
                    }
                    InsightRow {
                        icon: "ti ti-flag-3",
                        title: "Kill pressure",
                        body: "The right rail now supports the live raid snapshot without pretending to know encounter-level log details.",
                    }
                }
            }
            div { class: "table-actions",
                button { class: "secondary-link table-cta", r#type: "button", "Share raid plan" }
            }
        }
    }
}

#[component]
pub(crate) fn RaidDifficultyRow(progress: RaidDifficultyProgress) -> Element {
    let completion = if progress.total == 0 {
        0
    } else {
        ((u32::from(progress.completed) * 100) / u32::from(progress.total)).min(100) as u8
    };
    let color = parse_color(completion);
    let width = format!("{}%", completion);

    rsx! {
        div { class: "parse-row",
            span { class: "parse-boss", "{progress.label}" }
            div { class: "parse-bar-track",
                div {
                    class: "parse-bar-fill",
                    style: "width: {width}; background: {color};",
                }
            }
            span { class: "parse-pct mono", style: "color: {color};", "{completion}%" }
            span { class: "parse-spec text-muted", "{progress.summary}" }
            span { class: "parse-dps mono text-muted", "{progress.completed}/{progress.total}" }
        }
    }
}

#[component]
pub(crate) fn MythicTab(profile: CharacterProfileData) -> Element {
    let timed_keys = timed_key_count(&profile.mythic_runs);
    let best_route = best_route_name(&profile.mythic_runs);
    let last_affix = latest_affix(&profile.mythic_runs);

    rsx! {
        div { class: "tab-stack",
            div { class: "content-grid mythic-grid",
                section { class: "card-surface score-ring-card",
                    ScoreRing { score: profile.mythic_score, season: "TWW S4" }
                    div { class: "score-ring-copy",
                        h2 { "Mythic+ score" }
                        p { class: "text-muted",
                            "The score ring reflects the live public Raider.IO season score for this character."
                        }
                    }
                }
                div { class: "metrics-grid compact-metrics",
                    MetricCard {
                        label: "Timed keys",
                        value: format!("{timed_keys}/{}", profile.mythic_runs.len()),
                        sublabel: String::from("Timed runs in the recent public log"),
                    }
                    MetricCard {
                        label: "Recent keys",
                        value: profile.weekly_keys.to_string(),
                        sublabel: String::from("Recent public runs visible in Raider.IO"),
                    }
                    MetricCard {
                        label: "Best route",
                        value: best_route,
                        sublabel: String::from("Highest recent public key in the log below"),
                    }
                    MetricCard {
                        label: "Last affix",
                        value: last_affix,
                        sublabel: String::from("Affix on the most recent public run"),
                    }
                }
            }
            div { class: "content-grid two-up",
                section { class: "card-elevated mythic-log-card",
                    div { class: "section-heading compact",
                        h2 { "Recent public runs" }
                        p { class: "text-muted",
                            "The run log is ordered from the latest public completion timestamps returned by Raider.IO."
                        }
                    }
                    div { class: "table-panel",
                        if profile.mythic_runs.is_empty() {
                            div { class: "empty-state text-muted",
                                "No recent public Mythic+ runs were returned for this character."
                            }
                        } else {
                            for run in profile.mythic_runs.iter() {
                                MythicRunRow { run: run.clone() }
                            }
                        }
                    }
                    div { class: "table-actions",
                        button {
                            class: "secondary-link table-cta",
                            r#type: "button",
                            "Open public run log"
                        }
                    }
                }
                MythicIntelPanel { profile: profile.clone() }
            }
        }
    }
}

#[component]
fn MythicIntelPanel(profile: CharacterProfileData) -> Element {
    let mut active_tab = use_tab_panel_store(0).active_tab;
    let timed_keys = timed_key_count(&profile.mythic_runs);
    let best_level = best_level(&profile.mythic_runs);

    rsx! {
        section { class: "card-surface tab-side-panel mythic-side-panel",
            div { class: "section-heading compact",
                div { class: "note-tabs",
                    button {
                        class: if active_tab() == 0 { "note-chip active" } else { "note-chip" },
                        r#type: "button",
                        onclick: move |_| active_tab.set(0),
                        "Routes"
                    }
                    button {
                        class: if active_tab() == 1 { "note-chip active" } else { "note-chip" },
                        r#type: "button",
                        onclick: move |_| active_tab.set(1),
                        "Affixes"
                    }
                    button {
                        class: if active_tab() == 2 { "note-chip active" } else { "note-chip" },
                        r#type: "button",
                        onclick: move |_| active_tab.set(2),
                        "Utility"
                    }
                }
                h2 { "Route board" }
                p { class: "text-muted",
                    "The side rail now summarizes the live public Mythic+ log instead of pretending the data is weekly planner state."
                }
            }
            div { class: "panel-badge-row",
                span { class: "status-chip accent", "{profile.weekly_keys} recent" }
                span { class: "status-chip", "Best +{best_level}" }
                span { class: "status-chip", "{timed_keys} timed" }
            }
            div { class: "insight-list compact-insights",
                if active_tab() == 0 {
                    InsightRow {
                        icon: "ti ti-route-square",
                        title: "Route focus",
                        body: "Use the best exposed run in the live log below as the starting point for the next key push.",
                    }
                    InsightRow {
                        icon: "ti ti-map-pin",
                        title: "Dungeon spread",
                        body: "The public run list is now the source of truth for which dungeons this character is actively pushing.",
                    }
                    InsightRow {
                        icon: "ti ti-flag-3",
                        title: "Weekly focus",
                        body: "Compare timed and overtime keys directly from the live public table before widening the push pool.",
                    }
                }
                if active_tab() == 1 {
                    InsightRow {
                        icon: "ti ti-bolt",
                        title: "Affix snapshot",
                        body: "The affix labels on recent runs reflect what Raider.IO returned for this profile, not a seeded weekly script.",
                    }
                    InsightRow {
                        icon: "ti ti-shield-lock",
                        title: "Route stability",
                        body: "Look for repeated affix and dungeon combinations in the live run log before planning the next route.",
                    }
                    InsightRow {
                        icon: "ti ti-clock-hour-9",
                        title: "Timer trim",
                        body: "Use timed versus over results in the public run list to spot where timer losses are happening.",
                    }
                }
                if active_tab() == 2 {
                    InsightRow {
                        icon: "ti ti-heart-handshake",
                        title: "Live utility view",
                        body: "This panel now complements the run table instead of inventing dungeon-specific coaching for a mock character.",
                    }
                    InsightRow {
                        icon: "ti ti-sparkles",
                        title: "Cooldown planning",
                        body: "Use the live run spread to decide which utilities matter most for the next real dungeon target.",
                    }
                    InsightRow {
                        icon: "ti ti-repeat",
                        title: "Reset discipline",
                        body: "Public run history is a better guide for reset calls than any hardcoded weekly recommendation.",
                    }
                }
            }
            div { class: "table-actions",
                button { class: "secondary-link table-cta", r#type: "button", "Pin route notes" }
            }
        }
    }
}

#[component]
pub(crate) fn ScoreRing(score: u32, season: &'static str) -> Element {
    let circumference = 2.0 * std::f64::consts::PI * 54.0;
    let fill = (score as f64 / 4000.0).min(1.0) * circumference;
    let gap = circumference - fill;
    let dasharray = format!("{fill:.2} {gap:.2}");

    rsx! {
        div { class: "score-ring-wrap",
            svg { width: "140", height: "140", view_box: "0 0 140 140",
                circle {
                    cx: "70",
                    cy: "70",
                    r: "54",
                    fill: "none",
                    stroke: "var(--color-border-subtle)",
                    stroke_width: "8",
                }
                circle {
                    cx: "70",
                    cy: "70",
                    r: "54",
                    fill: "none",
                    stroke: "var(--color-accent)",
                    stroke_width: "8",
                    stroke_dasharray: "{dasharray}",
                    stroke_linecap: "round",
                    transform: "rotate(-90 70 70)",
                }
                text {
                    x: "70",
                    y: "68",
                    text_anchor: "middle",
                    font_family: "'JetBrains Mono', monospace",
                    font_size: "22",
                    fill: "var(--color-accent)",
                    "{score}"
                }
                text {
                    x: "70",
                    y: "84",
                    text_anchor: "middle",
                    font_family: "'Inter', sans-serif",
                    font_size: "11",
                    fill: "var(--color-text-muted)",
                    "{season}"
                }
            }
        }
    }
}

#[component]
pub(crate) fn MythicRunRow(run: MythicRun) -> Element {
    let result_class = if run.result == "Timed" { "result-win" } else { "result-loss" };
    let dungeon_icon = mythic_dungeon_icon(&run.dungeon);

    rsx! {
        div { class: "table-row mythic-row",
            div { class: "mythic-dungeon",
                if let Some(icon_url) = run.icon_url.as_ref() {
                    img {
                        class: "mythic-run-image",
                        src: "{icon_url}",
                        alt: "{run.dungeon} icon",
                    }
                } else {
                    i { class: "{dungeon_icon}" }
                }
                span { "{run.dungeon}" }
            }
            span { class: "mono", "+{run.level}" }
            span { class: "result-badge {result_class}", "{run.result}" }
            span { class: "text-muted mono", "{run.timing}" }
            span { class: "affix-chip", "{run.affix}" }
        }
    }
}

fn timed_key_count(runs: &[MythicRun]) -> usize {
    runs.iter().filter(|run| run.result == "Timed").count()
}

fn best_route_name(runs: &[MythicRun]) -> String {
    runs.iter()
        .max_by_key(|run| run.level)
        .map(|run| run.dungeon.clone())
        .unwrap_or_else(|| String::from("No runs"))
}

fn latest_affix(runs: &[MythicRun]) -> String {
    runs.first()
        .map(|run| run.affix.clone())
        .unwrap_or_else(|| String::from("Unknown"))
}

fn best_level(runs: &[MythicRun]) -> u8 {
    runs.iter().map(|run| run.level).max().unwrap_or_default()
}