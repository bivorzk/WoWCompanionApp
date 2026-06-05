use dioxus::prelude::*;

use crate::{
    services::{
        gear_art_class, gear_quality_label, gear_slot_icon, gear_upgrade_tip,
        gear_upgrade_track, quality_color, spec_icon,
    },
    stores::use_gear_inspector_store,
    types::{CharacterProfileData, GearSlotData, ProfileTab},
};

use super::common::FactionBadge;

#[component]
pub(crate) fn CharacterHeader(profile: CharacterProfileData) -> Element {
    let spec_glyph = spec_icon(&profile.class_name, &profile.spec);

    rsx! {
        header { class: "char-header",
            div { class: "avatar-shell",
                div { class: "avatar-ring",
                    if let Some(avatar_url) = profile.avatar_url.as_ref() {
                        img {
                            class: "char-avatar char-avatar-image",
                            src: "{avatar_url}",
                            alt: "{profile.name} portrait",
                        }
                    } else {
                        span { class: "char-avatar", "{profile.initials()}" }
                    }
                }
                span { class: "avatar-badge", "{profile.avatar_badge}" }
                span { class: "presence-dot online" }
            }
            div { class: "char-info",
                span { class: "eyebrow", "{profile.title}" }
                div { class: "name-row",
                    h1 { class: "char-name", "{profile.name}" }
                    FactionBadge { faction: profile.faction }
                    span { class: "status-chip online", "Online" }
                }
                div { class: "identity-row",
                    span { class: "class-badge",
                        i { class: "{spec_glyph}" }
                        span { "{profile.spec} {profile.class_name}" }
                    }
                    span { class: "char-realm", "{profile.realm} / {profile.region}" }
                    span { class: "char-realm", "{profile.guild}" }
                }
                div { class: "status-strip",
                    span { class: "status-chip accent", "{profile.raid_progress} raid" }
                    span { class: "status-chip", "Season 4 live" }
                    span { class: "status-chip", "{profile.hero_talent}" }
                }
                div { class: "quick-stats",
                    StatPill { label: "ilvl", value: profile.item_level.to_string() }
                    StatPill { label: "M+", value: profile.mythic_score.to_string() }
                    StatPill { label: "PvP", value: profile.pvp_rating.to_string() }
                    StatPill {
                        label: "Honor",
                        value: profile.honor_level.to_string(),
                    }
                }
            }
            div { class: "header-summary card-surface",
                span { class: "metric-label", "Season" }
                strong { class: "header-summary-title", "{profile.season_summary}" }
                p { class: "text-muted", "{profile.hero_talent}" }
                div { class: "summary-kpis",
                    div {
                        span { class: "metric-label", "Wins" }
                        strong { class: "mono", "{profile.arena_win_rate}%" }
                    }
                    div {
                        span { class: "metric-label", "Kills" }
                        strong { class: "mono", "{profile.raid_progress}" }
                    }
                }
            }
        }
    }
}

#[component]
fn StatPill(label: &'static str, value: String) -> Element {
    rsx! {
        div { class: "stat-pill",
            span { class: "stat-label", "{label}" }
            span { class: "stat-value mono", "{value}" }
        }
    }
}

#[component]
pub(crate) fn ProfileSidebar(profile: CharacterProfileData) -> Element {
    let inspector = use_gear_inspector_store();
    let selected_slot = inspector.selected_slot;
    let mut show_upgrade_finder = inspector.show_upgrade_finder;
    let next_breakpoint = 535u16;
    let breakpoint_progress =
        ((profile.item_level.min(next_breakpoint) as f32 / next_breakpoint as f32) * 100.0)
            .round();
    let breakpoint_width = format!("{breakpoint_progress:.0}%");
    let upgraded_slots = profile
        .gear_slots
        .iter()
        .filter(|slot| slot.item_level >= 525)
        .count();
    let upgrade_badge = if upgraded_slots == 0 {
        String::from("No 525+ slots yet")
    } else {
        format!("{upgraded_slots} slots 525+")
    };
    let breakpoint_copy = if profile.item_level >= next_breakpoint {
        String::from("Next breakpoint secured")
    } else {
        format!("{} ilvl to {}", next_breakpoint - profile.item_level, next_breakpoint)
    };
    let equipped_slots = profile.gear_slots.len();
    let bis_aligned = profile
        .gear_slots
        .iter()
        .filter(|slot| slot.item_level >= 528)
        .count();
    let upgrade_targets = profile
        .gear_slots
        .iter()
        .filter(|slot| slot.item_level < 525)
        .take(3)
        .map(|slot| {
            format!(
                "{}: {}",
                slot.slot_name,
                gear_upgrade_tip(slot.slot_name.as_str())
            )
        })
        .collect::<Vec<_>>();

    rsx! {
        div { class: "sidebar-stack",
            div { class: "card-elevated sidebar-card gear-summary-card",
                div { class: "section-heading compact",
                    h2 { "Loadout" }
                    p { class: "text-muted",
                        "A clearer loadout summary gives the left rail more purpose before players inspect individual slots."
                    }
                }
                div { class: "gear-summary-top",
                    div {
                        span { class: "metric-label", "Item level" }
                        strong { class: "gear-summary-ilvl mono", "{profile.item_level}" }
                    }
                    div { class: "gear-summary-badges",
                        span { class: "status-pill accent", "{profile.raid_progress}" }
                        span { class: "status-pill", "{upgrade_badge}" }
                    }
                }
                div { class: "gear-progress-track",
                    div {
                        class: "gear-progress-fill",
                        style: "width: {breakpoint_width};",
                    }
                }
                div { class: "gear-progress-copy",
                    span { class: "text-muted", "Progress toward the next breakpoint" }
                    span { class: "mono", "{breakpoint_copy}" }
                }
                div { class: "gear-compare-grid",
                    div { class: "gear-compare-card",
                        span { class: "metric-label", "Equipped" }
                        strong { class: "mono", "{equipped_slots}/16" }
                    }
                    div { class: "gear-compare-card",
                        span { class: "metric-label", "Best in slot" }
                        strong { class: "mono", "{bis_aligned}/16" }
                    }
                }
                div { class: "summary-pills",
                    span { class: "summary-pill", "{profile.role}" }
                    span { class: "summary-pill", "{profile.hero_talent}" }
                    span { class: "summary-pill", "{profile.achievement_points} achievement pts" }
                }
                div { class: "gear-actions",
                    button {
                        class: if show_upgrade_finder() { "primary-link sidebar-cta" } else { "secondary-link sidebar-cta" },
                        r#type: "button",
                        onclick: move |_| show_upgrade_finder.set(!show_upgrade_finder()),
                        if show_upgrade_finder() {
                            "Hide upgrade finder"
                        } else {
                            "Open upgrade finder"
                        }
                    }
                }
                if show_upgrade_finder() {
                    div { class: "upgrade-finder card-surface",
                        div { class: "upgrade-finder-head",
                            span { class: "metric-label", "Suggested upgrades" }
                            span { class: "status-chip accent", "3 targets" }
                        }
                        div { class: "upgrade-finder-list",
                            for target in upgrade_targets.iter() {
                                div { class: "upgrade-target",
                                    i { class: "ti ti-chevron-right" }
                                    span { "{target}" }
                                }
                            }
                        }
                    }
                }
            }
            div { class: "card-surface sidebar-card",
                div { class: "section-heading compact",
                    h2 { "Gear inspector" }
                    p { class: "text-muted",
                        "Slot cards now carry quality, item names, and a stronger inspection state instead of flat placeholder squares."
                    }
                }
                div { class: "gear-list",
                    for slot in profile.gear_slots.iter() {
                        GearSlot { slot: slot.clone(), selected_slot }
                    }
                }
                if let Some(selected) = selected_slot() {
                    GearDetailCard { slot: selected, selected_slot }
                }
            }
        }
    }
}

#[component]
fn GearSlot(slot: GearSlotData, selected_slot: Signal<Option<GearSlotData>>) -> Element {
    let quality = quality_color(slot.quality);
    let slot_icon = gear_slot_icon(slot.slot_name.as_str());
    let art_class = gear_art_class(slot.slot_name.as_str());
    let inspected_slot = slot.clone();

    rsx! {
        button {
            class: "gear-slot",
            r#type: "button",
            title: "{slot.item_name} - ilvl {slot.item_level}",
            onclick: move |_| selected_slot.set(Some(inspected_slot.clone())),
            div { class: "gear-icon-shell",
                div {
                    class: "gear-icon gear-art {art_class}",
                    style: "border-color: {quality}; color: {quality};",
                    span { class: "gear-art-sheen" }
                    i { class: "{slot_icon}" }
                }
                span { class: "gear-slot-tag mono", "{slot.label}" }
            }
            div { class: "gear-info",
                div { class: "gear-title-row",
                    strong { class: "gear-item-name", "{slot.item_name}" }
                    span { class: "gear-ilvl mono", style: "color: {quality};", "{slot.item_level}" }
                }
                span { class: "gear-slot-label text-muted", "{slot.slot_name}" }
            }
        }
    }
}

#[component]
fn GearDetailCard(slot: GearSlotData, selected_slot: Signal<Option<GearSlotData>>) -> Element {
    let quality = quality_color(slot.quality);
    let quality_label = gear_quality_label(slot.quality);
    let slot_icon = gear_slot_icon(slot.slot_name.as_str());
    let art_class = gear_art_class(slot.slot_name.as_str());
    let track = gear_upgrade_track(slot.item_level);
    let upgrade_tip = gear_upgrade_tip(slot.slot_name.as_str());

    rsx! {
        section { class: "gear-detail-card card-elevated",
            div { class: "gear-detail-head",
                div { class: "gear-detail-media",
                    div {
                        class: "gear-icon gear-art gear-detail-icon {art_class}",
                        style: "border-color: {quality}; color: {quality};",
                        span { class: "gear-art-sheen" }
                        i { class: "{slot_icon}" }
                    }
                    div { class: "gear-detail-meta",
                        span {
                            class: "status-chip",
                            style: "color: {quality}; border-color: {quality};",
                            "{quality_label}"
                        }
                        strong { class: "gear-detail-name", "{slot.item_name}" }
                        span { class: "text-muted", "{slot.slot_name} - {track}" }
                    }
                }
                button {
                    class: "gear-detail-close",
                    r#type: "button",
                    onclick: move |_| selected_slot.set(None),
                    i { class: "ti ti-x" }
                }
            }
            div { class: "gear-detail-grid",
                div { class: "gear-detail-stat",
                    span { class: "metric-label", "Item level" }
                    strong { class: "mono", style: "color: {quality};", "{slot.item_level}" }
                }
                div { class: "gear-detail-stat",
                    span { class: "metric-label", "Source" }
                    strong { "{slot.source}" }
                }
            }
            p { class: "gear-detail-note text-muted", "{upgrade_tip}" }
        }
    }
}

#[component]
pub(crate) fn TabBar(active_tab: Signal<ProfileTab>) -> Element {
    rsx! {
        nav { class: "tab-bar",
            TabButton {
                label: "PvP",
                icon: "ti ti-swords",
                tab: ProfileTab::Pvp,
                active_tab,
            }
            TabButton {
                label: "Raids",
                icon: "ti ti-flame",
                tab: ProfileTab::Raids,
                active_tab,
            }
            TabButton {
                label: "Mythic+",
                icon: "ti ti-door-enter",
                tab: ProfileTab::Mythic,
                active_tab,
            }
            TabButton {
                label: "Endgame",
                icon: "ti ti-radar",
                tab: ProfileTab::Endgame,
                active_tab,
            }
            TabButton {
                label: "Collection",
                icon: "ti ti-medal",
                tab: ProfileTab::Collection,
                active_tab,
            }
        }
    }
}

#[component]
fn TabButton(
    label: &'static str,
    icon: &'static str,
    tab: ProfileTab,
    active_tab: Signal<ProfileTab>,
) -> Element {
    let is_active = active_tab() == tab;

    rsx! {
        button {
            class: if is_active { "tab-item active" } else { "tab-item" },
            r#type: "button",
            onclick: move |_| active_tab.set(tab),
            i { class: "{icon}" }
            span { "{label}" }
        }
    }
}