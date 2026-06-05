use crate::{
    types::{
        AchievementEntry, CharacterProfileData, CollectionAvailability, CollectionItem,
        FavoriteCharacter, Faction, GearSlotData, ItemQuality, MythicRun, PvpMapStat,
        RaidDifficultyProgress,
    },
    Route,
};

pub(crate) trait CompanionRepository {
    fn sample_profile_route(&self) -> Route;
    fn favorite_profiles(&self) -> Vec<FavoriteCharacter>;
    fn character_profile(&self, region: &str, realm: &str, name: &str) -> CharacterProfileData;
}

#[derive(Clone, Copy, Default)]
pub(crate) struct MockCompanionRepository;

impl CompanionRepository for MockCompanionRepository {
    fn sample_profile_route(&self) -> Route {
        Route::CharacterProfile {
            region: String::from("eu"),
            realm: String::from("ravencrest"),
            name: String::from("Bvb"),
        }
    }

    fn favorite_profiles(&self) -> Vec<FavoriteCharacter> {
        vec![
            FavoriteCharacter {
                route: Route::CharacterProfile {
                    region: String::from("eu"),
                    realm: String::from("ravencrest"),
                    name: String::from("Bvb"),
                },
                name: "Bvb",
                realm: "Ravencrest",
                region: "EU",
                faction: Faction::Alliance,
                class_name: "Paladin",
                spec: "Holy",
                item_level: 528,
                mythic_score: 3247,
                pvp_rating: 2481,
            },
            FavoriteCharacter {
                route: Route::CharacterProfile {
                    region: String::from("eu"),
                    realm: String::from("ravencrest"),
                    name: String::from("Bicmex"),
                },
                name: "Bicmex",
                realm: "Ravencrest",
                region: "EU",
                faction: Faction::Alliance,
                class_name: "Hunter",
                spec: "Marksmanship",
                item_level: 524,
                mythic_score: 3110,
                pvp_rating: 2174,
            },
        ]
    }

    fn character_profile(&self, region: &str, realm: &str, name: &str) -> CharacterProfileData {
        let display_name = display_name(name);

        CharacterProfileData {
            name: display_name.clone(),
            realm: display_slug(realm),
            region: region.to_ascii_uppercase(),
            faction: faction_for_character(&display_name),
            class_name: String::from("Paladin"),
            spec: String::from("Holy"),
            role: "Healer",
            title: "The Argent Witness",
            guild: String::from("<Dawn Vanguard>"),
            hero_talent: "Herald of the Sun",
            season_summary: "Ladder climb with raid utility on standby",
            avatar_badge: String::from("HP"),
            avatar_url: None,
            item_level: 528,
            mythic_score: 3247,
            pvp_rating: 2481,
            arena_win_rate: 62,
            honor_level: 418,
            honorable_kills: 24_816,
            weekly_keys: 7,
            mounts_collected: 463,
            pets_collected: 712,
            mounts_availability: CollectionAvailability::Available,
            pets_availability: CollectionAvailability::Available,
            achievements_unlocked: 3_186,
            achievement_points: 31_680,
            raid_tier: String::from("Liberation of Undermine"),
            raid_progress: String::from("8/8 H"),
            pvp_map_stats: vec![
                PvpMapStat { map: String::from("Hook Point"), played: 14, won: 9, lost: 5 },
                PvpMapStat { map: String::from("Mugambala"), played: 11, won: 7, lost: 4 },
                PvpMapStat { map: String::from("Nagrand Arena"), played: 9, won: 4, lost: 5 },
                PvpMapStat { map: String::from("Empyrean Domain"), played: 8, won: 5, lost: 3 },
            ],
            raid_difficulties: vec![
                RaidDifficultyProgress {
                    label: String::from("Normal"),
                    completed: 8,
                    total: 8,
                    summary: String::from("8/8 cleared"),
                },
                RaidDifficultyProgress {
                    label: String::from("Heroic"),
                    completed: 8,
                    total: 8,
                    summary: String::from("8/8 cleared"),
                },
                RaidDifficultyProgress {
                    label: String::from("Mythic"),
                    completed: 3,
                    total: 8,
                    summary: String::from("3/8 progressing"),
                },
            ],
            mythic_runs: vec![
                MythicRun { dungeon: String::from("The Stonevault"), level: 15, result: String::from("Timed"), timing: String::from("-02:11"), affix: String::from("Tyrannical"), icon_url: None },
                MythicRun { dungeon: String::from("Ara-Kara"), level: 14, result: String::from("Timed"), timing: String::from("-00:48"), affix: String::from("Fortified"), icon_url: None },
                MythicRun { dungeon: String::from("City of Threads"), level: 15, result: String::from("Over"), timing: String::from("+01:22"), affix: String::from("Tyrannical"), icon_url: None },
                MythicRun { dungeon: String::from("The Dawnbreaker"), level: 16, result: String::from("Timed"), timing: String::from("-01:03"), affix: String::from("Fortified"), icon_url: None },
            ],
            gear_slots: vec![
                GearSlotData { slot_name: String::from("Head"), label: String::from("HD"), item_name: String::from("Crown of Radiant Vigil"), source: String::from("Demo loadout"), item_level: 528, quality: ItemQuality::Epic },
                GearSlotData { slot_name: String::from("Neck"), label: String::from("NK"), item_name: String::from("Locket of Dawning Grace"), source: String::from("Demo loadout"), item_level: 525, quality: ItemQuality::Epic },
                GearSlotData { slot_name: String::from("Shoulders"), label: String::from("SH"), item_name: String::from("Sunforged Pauldrons"), source: String::from("Demo loadout"), item_level: 528, quality: ItemQuality::Legendary },
                GearSlotData { slot_name: String::from("Back"), label: String::from("BK"), item_name: String::from("Moonwoven Cape"), source: String::from("Demo loadout"), item_level: 522, quality: ItemQuality::Rare },
                GearSlotData { slot_name: String::from("Chest"), label: String::from("CH"), item_name: String::from("Breastplate of Second Sunrise"), source: String::from("Demo loadout"), item_level: 528, quality: ItemQuality::Epic },
                GearSlotData { slot_name: String::from("Wrists"), label: String::from("WR"), item_name: String::from("Gleamthread Bracers"), source: String::from("Demo loadout"), item_level: 522, quality: ItemQuality::Rare },
                GearSlotData { slot_name: String::from("Hands"), label: String::from("HN"), item_name: String::from("Sanctified Handguards"), source: String::from("Demo loadout"), item_level: 528, quality: ItemQuality::Epic },
                GearSlotData { slot_name: String::from("Waist"), label: String::from("WT"), item_name: String::from("Lifebound Girdle"), source: String::from("Demo loadout"), item_level: 525, quality: ItemQuality::Epic },
                GearSlotData { slot_name: String::from("Legs"), label: String::from("LG"), item_name: String::from("Legguards of the Last Beacon"), source: String::from("Demo loadout"), item_level: 528, quality: ItemQuality::Legendary },
                GearSlotData { slot_name: String::from("Feet"), label: String::from("FT"), item_name: String::from("Dawnmarch Sabatons"), source: String::from("Demo loadout"), item_level: 522, quality: ItemQuality::Rare },
                GearSlotData { slot_name: String::from("Ring 1"), label: String::from("R1"), item_name: String::from("Signet of Emberlight"), source: String::from("Demo loadout"), item_level: 525, quality: ItemQuality::Epic },
                GearSlotData { slot_name: String::from("Ring 2"), label: String::from("R2"), item_name: String::from("Loop of Quiet Mercy"), source: String::from("Demo loadout"), item_level: 522, quality: ItemQuality::Rare },
                GearSlotData { slot_name: String::from("Trinket 1"), label: String::from("T1"), item_name: String::from("Icon of Sacred Flame"), source: String::from("Demo loadout"), item_level: 528, quality: ItemQuality::Legendary },
                GearSlotData { slot_name: String::from("Trinket 2"), label: String::from("T2"), item_name: String::from("Seal of Blazing Prayer"), source: String::from("Demo loadout"), item_level: 525, quality: ItemQuality::Epic },
                GearSlotData { slot_name: String::from("Main Hand"), label: String::from("MH"), item_name: String::from("Mace of Radiant Chorus"), source: String::from("Demo loadout"), item_level: 528, quality: ItemQuality::Legendary },
                GearSlotData { slot_name: String::from("Off Hand"), label: String::from("OH"), item_name: String::from("Aegis of Hallowed Stars"), source: String::from("Demo loadout"), item_level: 525, quality: ItemQuality::Epic },
            ],
            mount_items: vec![
                CollectionItem { label: String::from("GR"), name: String::from("Gilded Gryphon"), source: String::from("Mount collection"), collected: true },
                CollectionItem { label: String::from("SK"), name: String::from("Skyreaver Kite"), source: String::from("Mount collection"), collected: true },
                CollectionItem { label: String::from("HM"), name: String::from("Hallowfall Mare"), source: String::from("Mount collection"), collected: true },
                CollectionItem { label: String::from("SB"), name: String::from("Stormbound Phoenix"), source: String::from("Mount collection"), collected: true },
            ],
            pet_items: vec![
                CollectionItem { label: String::from("CH"), name: String::from("Core Hound Pup"), source: String::from("Pet collection"), collected: true },
                CollectionItem { label: String::from("CR"), name: String::from("Cinderweb Recluse"), source: String::from("Pet collection"), collected: true },
            ],
            achievements: vec![
                AchievementEntry { icon: "ti ti-trophy", name: String::from("Gladiator's Triumph"), description: String::from("Recent achievement event recorded on the character."), timestamp: String::from("2h ago") },
                AchievementEntry { icon: "ti ti-flame", name: String::from("Heroic: Chrome King"), description: String::from("Recent achievement event recorded on the character."), timestamp: String::from("Yesterday") },
                AchievementEntry { icon: "ti ti-door-enter", name: String::from("Keystone Hero"), description: String::from("Recent achievement event recorded on the character."), timestamp: String::from("3d ago") },
            ],
        }
    }
}

fn faction_for_character(name: &str) -> Faction {
    match name {
        "Thornvale" => Faction::Horde,
        _ => Faction::Alliance,
    }
}

fn display_name(value: &str) -> String {
    let cleaned = value.trim();
    if cleaned.is_empty() {
        return String::from("Bvb");
    }

    capitalize_word(cleaned)
}

fn display_slug(value: &str) -> String {
    let words = value
        .split(['-', '_', ' '])
        .filter(|part| !part.is_empty())
        .map(capitalize_word)
        .collect::<Vec<_>>();

    if words.is_empty() {
        String::from("Tarren Mill")
    } else {
        words.join(" ")
    }
}

fn capitalize_word(value: &str) -> String {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) => format!(
            "{}{}",
            first.to_ascii_uppercase(),
            chars.as_str().to_ascii_lowercase()
        ),
        None => String::new(),
    }
}