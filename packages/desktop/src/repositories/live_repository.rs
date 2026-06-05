use api::{
    blizzard::{
        BlizzardAchievementEvent, BlizzardClient, BlizzardCollectedMount,
        BlizzardCollectedPet, BlizzardEquippedItem, BlizzardPvPMapStatistic,
    },
    raiderio::{
        RaiderIoCharacter, RaiderIoCharacterOverviewQuery, RaiderIoClient, RaiderIoKeystoneRun,
        RaiderIoRaidProgressEntry,
    },
    ApiConfig,
};
use std::{collections::HashSet, time::{SystemTime, UNIX_EPOCH}};

use crate::{
    repositories::{CompanionRepository, MockCompanionRepository},
    types::{
        AchievementEntry, CharacterProfileData, CollectionAvailability, CollectionItem,
        Faction, GearSlotData, ItemQuality, MythicRun, PvpMapStat, RaidDifficultyProgress,
    },
    Route,
};

#[derive(Clone)]
pub(crate) struct LiveCompanionRepository {
    fallback: MockCompanionRepository,
    raiderio: RaiderIoClient,
    blizzard: BlizzardClient,
    api_config: ApiConfig,
}

impl Default for LiveCompanionRepository {
    fn default() -> Self {
        Self {
            fallback: MockCompanionRepository,
            raiderio: RaiderIoClient::new(),
            blizzard: BlizzardClient::new(),
            api_config: ApiConfig::default(),
        }
    }
}

impl LiveCompanionRepository {
    pub(crate) fn sample_profile_route(&self) -> Route {
        self.fallback.sample_profile_route()
    }

    pub(crate) async fn character_profile(
        &self,
        region: &str,
        realm: &str,
        name: &str,
    ) -> Result<CharacterProfileData, String> {
        let mut profile = self.fallback.character_profile(region, realm, name);
        let character = RaiderIoCharacter::new(region, realm, name);
        let query = RaiderIoCharacterOverviewQuery::new();

        profile.title = "Live profile snapshot";
        profile.hero_talent = "Public profile sync";
        profile.season_summary = "Live progression snapshot";
        profile.guild = String::from("No guild listed");
        profile.item_level = 0;
        profile.mythic_score = 0;
        profile.pvp_rating = 0;
        profile.arena_win_rate = 0;
        profile.honor_level = 0;
        profile.honorable_kills = 0;
        profile.weekly_keys = 0;
        profile.mounts_collected = 0;
        profile.pets_collected = 0;
        profile.mounts_availability = CollectionAvailability::Unavailable;
        profile.pets_availability = CollectionAvailability::Unavailable;
        profile.achievements_unlocked = 0;
        profile.achievement_points = 0;
        profile.raid_tier.clear();
        profile.raid_progress = String::from("No public raid progression");
        profile.pvp_map_stats.clear();
        profile.raid_difficulties.clear();
        profile.mythic_runs.clear();
        profile.gear_slots.clear();
        profile.mount_items.clear();
        profile.pet_items.clear();
        profile.achievements.clear();

        let basic = self
            .raiderio
            .fetch_character_profile_with_fields(
                region,
                realm,
                name,
                &["mythic_plus_recent_runs", "mythic_plus_scores_by_season:current"],
            )
            .await
            .map_err(api_error_to_string)?;

        profile.name = basic.name;
        profile.realm = basic.realm;
        profile.region = basic.region.to_ascii_uppercase();
        if let Some(class_name) = basic.class_name {
            profile.class_name = class_name;
        }
        if let Some(spec) = basic.active_spec_name {
            profile.spec = spec;
        }
        if let Some(achievement_points) = basic.achievement_points {
            profile.achievement_points = achievement_points;
        }
        profile.avatar_url = basic.thumbnail_url;

        if let Some(current_season_scores) = basic.mythic_plus_scores_by_season.first() {
            profile.mythic_score = current_season_scores
                .scores
                .all
                .round()
                .clamp(0.0, u32::MAX as f64) as u32;
        }

        if !basic.mythic_plus_recent_runs.is_empty() {
            let recent_runs = recent_public_runs(basic.mythic_plus_recent_runs.clone());
            profile.weekly_keys = recent_runs.len().min(u8::MAX as usize) as u8;

            let live_runs = recent_runs
                .into_iter()
                .take(6)
                .map(map_live_run)
                .collect::<Vec<_>>();

            if !live_runs.is_empty() {
                profile.mythic_runs = live_runs;
            }
        }

        if let Ok(details) = self.raiderio.fetch_detailed_character(&character, &query).await {
            profile.name = details.name;

            if let Some(realm) = details.realm {
                profile.realm = realm.name;
            }

            if let Some(region) = details.region {
                profile.region = region
                    .short_name
                    .unwrap_or(region.slug)
                    .to_ascii_uppercase();
            }

            if let Some(class_name) = details.class {
                profile.class_name = class_name.name;
            }

            if let Some(spec) = details.spec {
                profile.spec = spec.name;
            }

            if let Some(faction) = details.faction.as_deref().and_then(parse_faction_name) {
                profile.faction = faction;
            }

            if let Some(item_level) = details.item_level_equipped {
                profile.item_level = item_level.round().clamp(0.0, u16::MAX as f64) as u16;
            }

            if let Some(achievement_points) = details.achievement_points {
                profile.achievement_points = achievement_points;
            }

            if let Some(thumbnail_url) = details.thumbnail_url {
                profile.avatar_url = Some(thumbnail_url);
            }

            if let Some(guild_name) = details
                .guild
                .as_ref()
                .and_then(|guild| guild.get("name"))
                .and_then(|value| value.as_str())
            {
                profile.guild = guild_name.to_string();
            }
        }

        if let Ok(raid_progress) = self.raiderio.fetch_character_raid_progress(&character, &query).await {
            profile.raid_tier = format_raid_tier_label(&raid_progress.tier);
            if let Some(entry) = select_raid_progress_entry(&raid_progress.raid_progress) {
                profile.raid_progress = raid_progress_summary(entry);
                profile.raid_difficulties = map_raid_difficulties(entry);
            }
        }

        if let Ok(summary) = self
            .blizzard
            .fetch_character_pvp_summary(&self.api_config, region, realm, name)
            .await
        {
            profile.honor_level = summary.honor_level.min(u16::MAX as u32) as u16;
            profile.honorable_kills = summary.honorable_kills;
            let live_map_stats = summary
                .pvp_map_statistics
                .iter()
                .map(map_pvp_map_stat)
                .collect::<Vec<_>>();
            profile.arena_win_rate = aggregate_pvp_win_rate(&live_map_stats);
            profile.pvp_map_stats = live_map_stats;
        }

        for bracket_name in ["solo-shuffle", "3v3", "2v2", "rbg"] {
            if let Ok(bracket) = self
                .blizzard
                .fetch_character_pvp_bracket_statistics(&self.api_config, region, realm, name, bracket_name)
                .await
            {
                profile.pvp_rating = bracket.rating.min(u16::MAX as u32) as u16;
                if bracket.season_match_statistics.played > 0 {
                    profile.arena_win_rate = ((bracket.season_match_statistics.won * 100)
                        / bracket.season_match_statistics.played)
                        .min(u8::MAX as u32) as u8;
                }
                if let Some(faction) = parse_faction_name(&bracket.faction.name) {
                    profile.faction = faction;
                }
                break;
            }
        }

        if let Ok(equipment) = self
            .blizzard
            .fetch_character_equipment(&self.api_config, region, realm, name)
            .await
        {
            let mut gear_slots = equipment
                .equipped_items
                .iter()
                .map(map_equipped_item)
                .collect::<Vec<_>>();
            gear_slots.sort_by_key(|slot| gear_slot_order(&slot.slot_name));
            profile.gear_slots = gear_slots;
        }

        match self
            .blizzard
            .fetch_character_mounts_collection(&self.api_config, region, realm, name)
            .await
        {
            Ok(mounts) => {
                profile.mounts_availability = CollectionAvailability::Available;
                profile.mounts_collected = mounts.mounts.len().min(u16::MAX as usize) as u16;
                profile.mount_items = mounts.mounts.iter().take(12).map(map_mount_item).collect();
            }
            Err(error) => {
                profile.mounts_availability = classify_collection_availability(&error);
            }
        }

        match self
            .blizzard
            .fetch_character_pets_collection(&self.api_config, region, realm, name)
            .await
        {
            Ok(pets) => {
                profile.pets_availability = CollectionAvailability::Available;
                profile.pets_collected = pets.pets.len().min(u16::MAX as usize) as u16;
                profile.pet_items = pets.pets.iter().take(12).map(map_pet_item).collect();
            }
            Err(error) => {
                profile.pets_availability = classify_collection_availability(&error);
            }
        }

        if let Ok(achievements) = self
            .blizzard
            .fetch_character_achievements(&self.api_config, region, realm, name)
            .await
        {
            profile.achievements_unlocked = achievements.total_quantity;
            profile.achievement_points = achievements.total_points;
            profile.achievements = achievements
                .recent_events
                .into_iter()
                .take(4)
                .map(map_achievement_event)
                .collect();
        }

        profile.role = role_for_profile(&profile.class_name, &profile.spec);
        profile.avatar_badge = avatar_badge(&profile.name, &profile.spec, &profile.class_name);

        Ok(profile)
    }
}

fn api_error_to_string(error: api::ApiError) -> String {
    match error {
        api::ApiError::Request(request_error) => {
            match request_error.status().map(|status| status.as_u16()) {
                Some(400) | Some(404) => String::from(
                    "Character not found on Raider.IO. Check the character name, realm, and region, then try again.",
                ),
                Some(429) => String::from(
                    "Raider.IO is rate limiting requests right now. Wait a moment and try again.",
                ),
                _ => format!("Live profile request failed: {request_error}"),
            }
        }
        other => other.to_string(),
    }
}

fn classify_collection_availability(error: &api::ApiError) -> CollectionAvailability {
    match error {
        api::ApiError::Request(request_error) => match request_error.status().map(|status| status.as_u16()) {
            Some(401) | Some(403) | Some(404) => CollectionAvailability::Restricted,
            _ => CollectionAvailability::Unavailable,
        },
        _ => CollectionAvailability::Unavailable,
    }
}

fn parse_faction_name(value: &str) -> Option<Faction> {
    match value.trim() {
        value if value.eq_ignore_ascii_case("Alliance") => Some(Faction::Alliance),
        value if value.eq_ignore_ascii_case("Horde") => Some(Faction::Horde),
        _ => None,
    }
}

fn role_for_profile(class_name: &str, spec_name: &str) -> &'static str {
    match (class_name, spec_name) {
        (_, "Holy") | (_, "Restoration") | (_, "Mistweaver") | (_, "Preservation") => {
            "Healer"
        }
        (_, "Protection")
        | (_, "Guardian")
        | (_, "Brewmaster")
        | (_, "Blood")
        | (_, "Vengeance") => "Tank",
        _ => "Damage",
    }
}

fn avatar_badge(name: &str, spec_name: &str, class_name: &str) -> String {
    let first = spec_name.chars().next().or_else(|| class_name.chars().next());
    let second = name.chars().next();

    match (first, second) {
        (Some(first), Some(second)) => {
            format!("{}{}", first.to_ascii_uppercase(), second.to_ascii_uppercase())
        }
        (Some(first), None) => first.to_ascii_uppercase().to_string(),
        _ => String::from("WC"),
    }
}

fn map_pvp_map_stat(stat: &BlizzardPvPMapStatistic) -> PvpMapStat {
    PvpMapStat {
        map: stat.world_map.name.clone(),
        played: stat.match_statistics.played.min(u16::MAX as u32) as u16,
        won: stat.match_statistics.won.min(u16::MAX as u32) as u16,
        lost: stat.match_statistics.lost.min(u16::MAX as u32) as u16,
    }
}

fn aggregate_pvp_win_rate(stats: &[PvpMapStat]) -> u8 {
    let played = stats.iter().map(|stat| u32::from(stat.played)).sum::<u32>();
    let won = stats.iter().map(|stat| u32::from(stat.won)).sum::<u32>();

    if played == 0 {
        0
    } else {
        ((won * 100) / played).min(u8::MAX as u32) as u8
    }
}

fn select_raid_progress_entry(
    entries: &[RaiderIoRaidProgressEntry],
) -> Option<&RaiderIoRaidProgressEntry> {
    entries.iter().max_by_key(|entry| {
        (
            mythic_bosses_killed(entry),
            heroic_bosses_killed(entry),
            normal_bosses_killed(entry),
            raid_total_bosses(entry),
        )
    })
}

fn raid_progress_summary(entry: &RaiderIoRaidProgressEntry) -> String {
    if let Some(summary) = entry.summary.as_ref().filter(|summary| !summary.trim().is_empty()) {
        return summary.clone();
    }

    let total = raid_total_bosses(entry);
    for (label, completed) in [
        ("M", mythic_bosses_killed(entry)),
        ("H", heroic_bosses_killed(entry)),
        ("N", normal_bosses_killed(entry)),
    ] {
        if completed > 0 {
            return format!("{completed}/{total} {label}");
        }
    }

    if total > 0 {
        format!("0/{total}")
    } else {
        String::from("No public raid progression")
    }
}

fn map_raid_difficulties(entry: &RaiderIoRaidProgressEntry) -> Vec<RaidDifficultyProgress> {
    let total = raid_total_bosses(entry);
    if total == 0 {
        return Vec::new();
    }

    let mut difficulties = Vec::new();
    push_raid_difficulty(
        &mut difficulties,
        "Normal",
        Some(u32::from(normal_bosses_killed(entry))),
        total,
    );
    push_raid_difficulty(
        &mut difficulties,
        "Heroic",
        Some(u32::from(heroic_bosses_killed(entry))),
        total,
    );
    push_raid_difficulty(
        &mut difficulties,
        "Mythic",
        Some(u32::from(mythic_bosses_killed(entry))),
        total,
    );
    difficulties
}

fn normal_bosses_killed(entry: &RaiderIoRaidProgressEntry) -> u8 {
    entry
        .normal_bosses_killed
        .unwrap_or(entry.encounters_defeated.normal.len() as u32)
        .min(u8::MAX as u32) as u8
}

fn heroic_bosses_killed(entry: &RaiderIoRaidProgressEntry) -> u8 {
    entry
        .heroic_bosses_killed
        .unwrap_or(entry.encounters_defeated.heroic.len() as u32)
        .min(u8::MAX as u32) as u8
}

fn mythic_bosses_killed(entry: &RaiderIoRaidProgressEntry) -> u8 {
    entry
        .mythic_bosses_killed
        .unwrap_or(entry.encounters_defeated.mythic.len() as u32)
        .min(u8::MAX as u32) as u8
}

fn raid_total_bosses(entry: &RaiderIoRaidProgressEntry) -> u8 {
    entry
        .total_bosses
        .map(|total| total.min(u8::MAX as u32) as u8)
        .or_else(|| entry.raid.as_deref().and_then(known_raid_total))
        .unwrap_or_else(|| {
            [
                normal_bosses_killed(entry),
                heroic_bosses_killed(entry),
                mythic_bosses_killed(entry),
            ]
            .into_iter()
            .max()
            .unwrap_or_default()
        })
}

fn known_raid_total(raid_slug: &str) -> Option<u8> {
    match raid_slug.trim() {
        "tier-tww-1" | "tier-tww-2" | "tier-tww-3" | "tier-mn-1" => Some(8),
        _ => None,
    }
}

fn format_raid_tier_label(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    if trimmed.chars().all(|character| character.is_ascii_digit()) {
        format!("Tier {trimmed}")
    } else {
        trimmed.to_string()
    }
}

fn push_raid_difficulty(
    difficulties: &mut Vec<RaidDifficultyProgress>,
    label: &str,
    completed: Option<u32>,
    total: u8,
) {
    let completed = completed.unwrap_or_default().min(u8::MAX as u32) as u8;
    if completed == 0 && total == 0 {
        return;
    }

    difficulties.push(RaidDifficultyProgress {
        label: label.to_string(),
        completed,
        total,
        summary: format!("{completed}/{total} cleared"),
    });
}

fn map_equipped_item(item: &BlizzardEquippedItem) -> GearSlotData {
    let slot_name = item.slot.name.clone();

    GearSlotData {
        label: slot_label(&slot_name),
        slot_name: slot_name.clone(),
        item_name: item.name.clone(),
        source: format!("Blizzard equipment profile · {}", item.quality.name),
        item_level: item
            .level
            .as_ref()
            .map(|level| level.value.min(u16::MAX as u32) as u16)
            .unwrap_or_default(),
        quality: map_item_quality(&item.quality.r#type),
    }
}

fn map_item_quality(value: &str) -> ItemQuality {
    match value.trim() {
        value if value.eq_ignore_ascii_case("LEGENDARY") => ItemQuality::Legendary,
        value if value.eq_ignore_ascii_case("EPIC") => ItemQuality::Epic,
        _ => ItemQuality::Rare,
    }
}

fn slot_label(slot_name: &str) -> String {
    match slot_name {
        "Head" => String::from("HD"),
        "Neck" => String::from("NK"),
        "Shoulders" => String::from("SH"),
        "Back" => String::from("BK"),
        "Chest" => String::from("CH"),
        "Wrists" => String::from("WR"),
        "Hands" => String::from("HN"),
        "Waist" => String::from("WT"),
        "Legs" => String::from("LG"),
        "Feet" => String::from("FT"),
        "Ring 1" => String::from("R1"),
        "Ring 2" => String::from("R2"),
        "Trinket 1" => String::from("T1"),
        "Trinket 2" => String::from("T2"),
        "Main Hand" => String::from("MH"),
        "Off Hand" => String::from("OH"),
        _ => slot_name
            .chars()
            .filter(|character| character.is_ascii_alphabetic())
            .take(2)
            .collect::<String>()
            .to_ascii_uppercase(),
    }
}

fn gear_slot_order(slot_name: &str) -> usize {
    match slot_name {
        "Head" => 0,
        "Neck" => 1,
        "Shoulders" => 2,
        "Back" => 3,
        "Chest" => 4,
        "Wrists" => 5,
        "Hands" => 6,
        "Waist" => 7,
        "Legs" => 8,
        "Feet" => 9,
        "Ring 1" => 10,
        "Ring 2" => 11,
        "Trinket 1" => 12,
        "Trinket 2" => 13,
        "Main Hand" => 14,
        "Off Hand" => 15,
        _ => usize::MAX,
    }
}

fn map_mount_item(item: &BlizzardCollectedMount) -> CollectionItem {
    CollectionItem {
        label: initials_label(&item.mount.name),
        name: item.mount.name.clone(),
        source: if item.is_useable {
            String::from("Blizzard mount collection · usable")
        } else {
            String::from("Blizzard mount collection · locked for this character")
        },
        collected: true,
    }
}

fn map_pet_item(item: &BlizzardCollectedPet) -> CollectionItem {
    CollectionItem {
        label: initials_label(&item.species.name),
        name: item.species.name.clone(),
        source: format!("Blizzard pet journal · level {}", item.level),
        collected: true,
    }
}

fn initials_label(value: &str) -> String {
    let parts = value
        .split_whitespace()
        .filter_map(|part| part.chars().next())
        .take(2)
        .collect::<String>();

    if parts.is_empty() {
        value.chars().take(2).collect::<String>().to_ascii_uppercase()
    } else {
        parts.to_ascii_uppercase()
    }
}

fn map_achievement_event(event: BlizzardAchievementEvent) -> AchievementEntry {
    AchievementEntry {
        icon: "ti ti-trophy",
        name: event.achievement.name,
        description: String::from("Recent achievement event from the Blizzard character profile."),
        timestamp: format_relative_timestamp(event.timestamp),
    }
}

fn format_relative_timestamp(timestamp_ms: u64) -> String {
    let now_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or(timestamp_ms);

    let elapsed = now_ms.saturating_sub(timestamp_ms);
    let minutes = elapsed / 60_000;
    let hours = elapsed / 3_600_000;
    let days = elapsed / 86_400_000;

    if minutes < 60 {
        format!("{minutes}m ago")
    } else if hours < 24 {
        format!("{hours}h ago")
    } else {
        format!("{days}d ago")
    }
}

fn recent_public_runs(mut runs: Vec<RaiderIoKeystoneRun>) -> Vec<RaiderIoKeystoneRun> {
    runs.sort_by(|left, right| right.completed_at.cmp(&left.completed_at));

    let mut seen_run_ids = HashSet::new();
    runs.into_iter()
        .filter(|run| seen_run_ids.insert(run.keystone_run_id))
        .collect()
}

fn map_live_run(run: RaiderIoKeystoneRun) -> MythicRun {
    let timed = run.clear_time_ms <= run.par_time_ms;
    let delta_ms = if timed {
        run.par_time_ms.saturating_sub(run.clear_time_ms)
    } else {
        run.clear_time_ms.saturating_sub(run.par_time_ms)
    };
    let delta_seconds = delta_ms / 1000;
    let delta_minutes = delta_seconds / 60;
    let remaining_seconds = delta_seconds % 60;
    let timing_prefix = if timed { '-' } else { '+' };
    let affix = run
        .affixes
        .first()
        .map(|affix| affix.name.clone())
        .unwrap_or_else(|| String::from("Unknown"));

    MythicRun {
        dungeon: run.short_name,
        level: run.mythic_level.min(u8::MAX as u32) as u8,
        result: if timed {
            String::from("Timed")
        } else {
            String::from("Over")
        },
        timing: format!("{timing_prefix}{delta_minutes:02}:{remaining_seconds:02}"),
        affix,
        icon_url: Some(run.icon_url),
    }
}