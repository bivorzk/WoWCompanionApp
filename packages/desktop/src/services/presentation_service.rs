use crate::types::ItemQuality;

pub(crate) fn parse_color(percentile: u8) -> &'static str {
    match percentile {
        95..=100 => "var(--color-parse-s)",
        75..=94 => "var(--color-parse-a)",
        50..=74 => "var(--color-parse-b)",
        25..=49 => "var(--color-parse-c)",
        _ => "var(--color-parse-d)",
    }
}

pub(crate) fn quality_color(quality: ItemQuality) -> &'static str {
    match quality {
        ItemQuality::Legendary => "var(--color-parse-s)",
        ItemQuality::Epic => "var(--color-ilvl-high)",
        ItemQuality::Rare => "var(--color-ilvl-mid)",
    }
}

pub(crate) fn spec_icon(class_name: &str, spec: &str) -> &'static str {
    match (class_name, spec) {
        ("Paladin", "Holy") => "ti ti-hammer",
        ("Paladin", _) => "ti ti-shield-half",
        ("Demon Hunter", _) => "ti ti-blade",
        ("Mage", _) => "ti ti-wand",
        ("Druid", _) => "ti ti-leaf",
        _ => "ti ti-star",
    }
}

pub(crate) fn gear_upgrade_tip(slot_name: &str) -> &'static str {
    match slot_name {
        "Back" => "Replace with a 525+ cloak from weekly keys or a vault pull.",
        "Wrists" => "Target a forged dungeon pair to smooth survivability without losing haste.",
        "Feet" => "A higher-track boot slot is the cleanest upgrade into the next breakpoint.",
        "Ring 2" => "Swap this ring for a crafted mastery/haste piece to stabilize output.",
        _ => "This slot is stable for now; bank crests for your weaker utility pieces first.",
    }
}

pub(crate) fn gear_quality_label(quality: ItemQuality) -> &'static str {
    match quality {
        ItemQuality::Legendary => "Legendary",
        ItemQuality::Epic => "Epic",
        ItemQuality::Rare => "Rare",
    }
}

pub(crate) fn gear_upgrade_track(item_level: u16) -> &'static str {
    match item_level {
        528..=u16::MAX => "Myth 6/6",
        525..=527 => "Myth 5/6",
        522..=524 => "Hero 6/6",
        _ => "Champion 8/8",
    }
}

pub(crate) fn gear_slot_icon(slot_name: &str) -> &'static str {
    match slot_name {
        "Head" => "ti ti-crown",
        "Neck" => "ti ti-diamond",
        "Ring 1" | "Ring 2" => "ti ti-circle",
        "Trinket 1" | "Trinket 2" => "ti ti-sparkles",
        "Main Hand" => "ti ti-sword",
        "Off Hand" => "ti ti-shield-half",
        _ => "ti ti-shield",
    }
}

pub(crate) fn gear_art_class(slot_name: &str) -> &'static str {
    match slot_name {
        "Head" | "Shoulders" | "Chest" | "Legs" | "Hands" => "art-radiant",
        "Neck" | "Ring 1" | "Ring 2" => "art-jewel",
        "Back" | "Feet" | "Waist" | "Wrists" => "art-ash",
        "Trinket 1" | "Trinket 2" => "art-relic",
        "Main Hand" | "Off Hand" => "art-arsenal",
        _ => "art-radiant",
    }
}

pub(crate) fn arena_map_icon(map: &str) -> &'static str {
    match map {
        "Hook Point" => "ti ti-anchor",
        "Mugambala" => "ti ti-building-castle",
        "Nagrand" => "ti ti-trees",
        "Empyrean" => "ti ti-cloud",
        "Tol'viron" => "ti ti-pyramid",
        _ => "ti ti-map-2",
    }
}

pub(crate) fn mythic_dungeon_icon(dungeon: &str) -> &'static str {
    match dungeon {
        "The Stonevault" => "ti ti-building-bank",
        "Ara-Kara" => "ti ti-spider",
        "City of Threads" => "ti ti-route-square",
        "The Dawnbreaker" => "ti ti-sun",
        _ => "ti ti-door-enter",
    }
}

pub(crate) fn sparkline_points(delta: i16) -> &'static str {
    match delta {
        20..=i16::MAX => "2,18 10,15 18,12 26,9 34,6 42,3",
        10..=19 => "2,17 10,15 18,13 26,11 34,9 42,7",
        1..=9 => "2,15 10,14 18,13 26,12 34,11 42,10",
        -9..=0 => "2,8 10,9 18,10 26,11 34,12 42,13",
        _ => "2,5 10,7 18,9 26,12 34,15 42,18",
    }
}

pub(crate) fn sparkline_end_y(delta: i16) -> &'static str {
    match delta {
        20..=i16::MAX => "3",
        10..=19 => "7",
        1..=9 => "10",
        -9..=0 => "13",
        _ => "18",
    }
}

pub(crate) fn metric_icon(label: &str) -> &'static str {
    match label {
        "ilvl" => "ti ti-shield",
        "M+" | "Score" | "Timed keys" | "Tracked keys" | "Timed pool" | "Best route" => {
            "ti ti-door-enter"
        }
        "PvP" | "Rated" | "Win rate" | "Honor" | "Honor level" | "Honorable kills" => {
            "ti ti-swords"
        }
        "Progress" | "Tier" | "Modes" => "ti ti-flame",
        "Mounts" | "Pets" | "Achievements" | "Points" => "ti ti-medal",
        "Role" => "ti ti-user",
        "Last affix" => "ti ti-bolt",
        _ => "ti ti-star",
    }
}

pub(crate) fn metric_trend(label: &str) -> (&'static str, &'static str) {
    match label {
        "ilvl" => ("Fresh upgrades", "up"),
        "M+" | "Score" | "Timed keys" | "Tracked keys" | "Timed pool" | "Best route" => {
            ("Dungeon active", "up")
        }
        "PvP" | "Rated" | "Win rate" | "Honor" | "Honor level" | "Honorable kills" => {
            ("Queue live", "up")
        }
        "Progress" | "Tier" | "Modes" => ("Raid live", "up"),
        "Mounts" | "Pets" | "Achievements" | "Points" => ("Account wide", "neutral"),
        "Role" | "Last affix" => ("Live profile", "neutral"),
        _ => ("Stable", "neutral"),
    }
}