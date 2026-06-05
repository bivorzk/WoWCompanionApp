use std::collections::BTreeMap;

use dioxus::prelude::*;

use crate::types::{CharacterProfileData, MythicRun, PvpMapStat, RaidDifficultyProgress};

use super::common::MetricCard;

#[component]
pub(crate) fn EndgameTab(profile: CharacterProfileData) -> Element {
    let total_matches = total_pvp_matches(&profile.pvp_map_stats);
    let total_wins = total_pvp_wins(&profile.pvp_map_stats);
    let total_losses = total_pvp_losses(&profile.pvp_map_stats);
    let map_count = profile.pvp_map_stats.len();
    let best_map = best_pvp_map(&profile.pvp_map_stats);
    let roughest_map = roughest_pvp_map(&profile.pvp_map_stats);
    let highest_clear = highest_raid_clear(&profile.raid_difficulties);
    let raid_focus = raid_focus_label(&profile.raid_difficulties);
    let raid_focus_note = raid_focus_note(&profile.raid_difficulties);
    let raid_boss_kills = total_raid_boss_kills(&profile.raid_difficulties);
    let timed_rate = timed_rate(&profile.mythic_runs);
    let average_key = average_key_level(&profile.mythic_runs);
    let best_recent_key = best_recent_key(&profile.mythic_runs);
    let route_anchor = most_played_dungeon(&profile.mythic_runs);
    let affix_anchor = dominant_affix(&profile.mythic_runs);
    let overtime_count = overtime_run_count(&profile.mythic_runs);
    let unique_dungeons = unique_dungeon_count(&profile.mythic_runs);

    rsx! {
        div { class: "tab-stack",
            div { class: "metrics-grid",
                MetricCard {
                    label: "PvP volume",
                    value: total_matches.to_string(),
                    sublabel: format!("{total_wins}-{total_losses} across {map_count} tracked maps"),
                }
                MetricCard {
                    label: "Raid focus",
                    value: raid_focus.clone(),
                    sublabel: highest_clear.clone(),
                }
                MetricCard {
                    label: "Mythic tempo",
                    value: format!("{timed_rate}%"),
                    sublabel: format!("Avg +{average_key} across {} recent runs", profile.mythic_runs.len()),
                }
                MetricCard {
                    label: "Route anchor",
                    value: route_anchor.clone(),
                    sublabel: affix_anchor.clone(),
                }
            }
            div { class: "endgame-grid",
                section { class: "card-elevated endgame-lane-card",
                    div { class: "section-heading compact",
                        h2 { "PvP deep dive" }
                        p { class: "text-muted",
                            "The PvP lane compresses public map results into a quick read on queue volume, safest maps, and the roughest current pressure points."
                        }
                    }
                    div { class: "panel-badge-row",
                        span { class: "status-chip accent",
                            if profile.pvp_rating > 0 {
                                "{profile.pvp_rating} rated"
                            } else {
                                "Unranked"
                            }
                        }
                        span { class: "status-chip", "{profile.arena_win_rate}% win rate" }
                        span { class: "status-chip", "Honor {profile.honor_level}" }
                    }
                    div { class: "endgame-lane-list",
                        EndgameDetail {
                            label: "Best map",
                            value: best_map.0,
                            note: best_map.1,
                        }
                        EndgameDetail {
                            label: "Pressure point",
                            value: roughest_map.0,
                            note: roughest_map.1,
                        }
                        EndgameDetail {
                            label: "Queue volume",
                            value: format!("{total_matches} matches"),
                            note: format!(
                                "{total_wins}-{total_losses} public W-L across the current Blizzard map record",
                            ),
                        }
                    }
                }
                section { class: "card-elevated endgame-lane-card",
                    div { class: "section-heading compact",
                        h2 { "Raid deep dive" }
                        p { class: "text-muted",
                            "The raid lane highlights the best public clear, the current progression lane, and how much raid work is still left in the visible difficulty spread."
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
                    div { class: "endgame-lane-list",
                        EndgameDetail {
                            label: "Highest clear",
                            value: highest_clear,
                            note: format!(
                                "{} raid modes with public progress visible",
                                profile.raid_difficulties.len(),
                            ),
                        }
                        EndgameDetail {
                            label: "Progression lane",
                            value: raid_focus,
                            note: raid_focus_note,
                        }
                        EndgameDetail {
                            label: "Bosses tracked",
                            value: raid_boss_kills.to_string(),
                            note: String::from("Total public boss kills summed across the visible raid difficulties."),
                        }
                    }
                }
                section { class: "card-elevated endgame-lane-card",
                    div { class: "section-heading compact",
                        h2 { "Mythic+ deep dive" }
                        p { class: "text-muted",
                            "The Mythic lane turns the recent Raider.IO log into a quick view of route repetition, timer health, and the affix pattern currently shaping the run pool."
                        }
                    }
                    div { class: "panel-badge-row",
                        span { class: "status-chip accent", "{profile.mythic_score} score" }
                        span { class: "status-chip", "Best +{best_recent_key}" }
                        span { class: "status-chip", "{profile.mythic_runs.len()} recent" }
                    }
                    div { class: "endgame-lane-list",
                        EndgameDetail {
                            label: "Route anchor",
                            value: route_anchor,
                            note: format!("{unique_dungeons} unique dungeons appear in the recent public sample"),
                        }
                        EndgameDetail {
                            label: "Timer health",
                            value: format!("{timed_rate}% timed"),
                            note: format!("{overtime_count} runs finished over timer in the current public log"),
                        }
                        EndgameDetail {
                            label: "Affix lane",
                            value: affix_anchor,
                            note: format!("Average key level sits around +{average_key} in the recent run pool"),
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn EndgameDetail(label: &'static str, value: String, note: String) -> Element {
    rsx! {
        div { class: "endgame-detail",
            div { class: "endgame-detail-head",
                span { class: "endgame-detail-label", "{label}" }
                strong { class: "endgame-detail-value", "{value}" }
            }
            p { class: "text-muted endgame-detail-note", "{note}" }
        }
    }
}

fn total_pvp_matches(stats: &[PvpMapStat]) -> u32 {
    stats.iter().map(|stat| u32::from(stat.played)).sum()
}

fn total_pvp_wins(stats: &[PvpMapStat]) -> u32 {
    stats.iter().map(|stat| u32::from(stat.won)).sum()
}

fn total_pvp_losses(stats: &[PvpMapStat]) -> u32 {
    stats.iter().map(|stat| u32::from(stat.lost)).sum()
}

fn best_pvp_map(stats: &[PvpMapStat]) -> (String, String) {
    stats.iter()
        .max_by_key(|stat| (win_rate_basis(stat), net_wins(stat), stat.played))
        .map(|stat| {
            (
                stat.map.clone(),
                format!("{}-{} W-L across {} matches", stat.won, stat.lost, stat.played),
            )
        })
        .unwrap_or_else(|| {
            (
                String::from("No map data"),
                String::from("No public PvP map statistics were returned for this character."),
            )
        })
}

fn roughest_pvp_map(stats: &[PvpMapStat]) -> (String, String) {
    stats.iter()
        .min_by_key(|stat| (net_wins(stat), -(i32::from(stat.played))))
        .map(|stat| {
            (
                stat.map.clone(),
                format!("{}-{} W-L across {} matches", stat.won, stat.lost, stat.played),
            )
        })
        .unwrap_or_else(|| {
            (
                String::from("No map data"),
                String::from("No public PvP map statistics were returned for this character."),
            )
        })
}

fn win_rate_basis(stat: &PvpMapStat) -> u32 {
    if stat.played == 0 {
        0
    } else {
        (u32::from(stat.won) * 1000) / u32::from(stat.played)
    }
}

fn net_wins(stat: &PvpMapStat) -> i32 {
    i32::from(stat.won) - i32::from(stat.lost)
}

fn highest_raid_clear(progress: &[RaidDifficultyProgress]) -> String {
    progress
        .iter()
        .filter(|entry| entry.completed > 0)
        .max_by_key(|entry| (raid_difficulty_rank(&entry.label), entry.completed))
        .map(|entry| format!("{} {}/{}", entry.label, entry.completed, entry.total))
        .unwrap_or_else(|| String::from("No raid data"))
}

fn raid_focus_label(progress: &[RaidDifficultyProgress]) -> String {
    progress
        .iter()
        .filter(|entry| entry.total > 0)
        .filter(|entry| entry.completed < entry.total)
        .max_by_key(|entry| (raid_difficulty_rank(&entry.label), entry.completed))
        .map(|entry| format!("{} progression", entry.label))
        .or_else(|| {
            progress
                .iter()
                .filter(|entry| entry.completed > 0)
                .max_by_key(|entry| (raid_difficulty_rank(&entry.label), entry.completed))
                .map(|entry| format!("{} farm", entry.label))
        })
        .unwrap_or_else(|| String::from("No raid data"))
}

fn raid_focus_note(progress: &[RaidDifficultyProgress]) -> String {
    progress
        .iter()
        .filter(|entry| entry.total > 0)
        .filter(|entry| entry.completed < entry.total)
        .max_by_key(|entry| (raid_difficulty_rank(&entry.label), entry.completed))
        .map(|entry| {
            format!(
                "{} bosses remain at {} difficulty according to the live public clear spread.",
                entry.total.saturating_sub(entry.completed),
                entry.label
            )
        })
        .unwrap_or_else(|| String::from("Every visible raid difficulty is either empty or fully cleared in the public data."))
}

fn total_raid_boss_kills(progress: &[RaidDifficultyProgress]) -> u32 {
    progress.iter().map(|entry| u32::from(entry.completed)).sum()
}

fn raid_difficulty_rank(label: &str) -> u8 {
    match label {
        "Mythic" => 3,
        "Heroic" => 2,
        "Normal" => 1,
        _ => 0,
    }
}

fn timed_rate(runs: &[MythicRun]) -> u8 {
    if runs.is_empty() {
        0
    } else {
        ((timed_run_count(runs) as u32 * 100) / runs.len() as u32) as u8
    }
}

fn timed_run_count(runs: &[MythicRun]) -> usize {
    runs.iter().filter(|run| run.result == "Timed").count()
}

fn overtime_run_count(runs: &[MythicRun]) -> usize {
    runs.iter().filter(|run| run.result != "Timed").count()
}

fn average_key_level(runs: &[MythicRun]) -> u8 {
    if runs.is_empty() {
        0
    } else {
        let total = runs.iter().map(|run| u32::from(run.level)).sum::<u32>();
        (total / runs.len() as u32).min(u8::MAX as u32) as u8
    }
}

fn best_recent_key(runs: &[MythicRun]) -> u8 {
    runs.iter().map(|run| run.level).max().unwrap_or_default()
}

fn most_played_dungeon(runs: &[MythicRun]) -> String {
    most_common_label(runs.iter().map(|run| run.dungeon.as_str()))
        .unwrap_or_else(|| String::from("No route data"))
}

fn dominant_affix(runs: &[MythicRun]) -> String {
    most_common_label(runs.iter().map(|run| run.affix.as_str()))
        .unwrap_or_else(|| String::from("Unknown affix"))
}

fn unique_dungeon_count(runs: &[MythicRun]) -> usize {
    let mut counts = BTreeMap::<&str, usize>::new();
    for run in runs {
        *counts.entry(run.dungeon.as_str()).or_default() += 1;
    }
    counts.len()
}

fn most_common_label<'a>(values: impl Iterator<Item = &'a str>) -> Option<String> {
    let mut counts = BTreeMap::<&'a str, usize>::new();
    for value in values.filter(|value| !value.trim().is_empty()) {
        *counts.entry(value).or_default() += 1;
    }

    counts
        .into_iter()
        .max_by_key(|(value, count)| (*count, *value))
        .map(|(value, _)| value.to_string())
}