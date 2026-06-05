use crate::Route;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum NavSection {
    Home,
    Overall,
    Character,
    Favorites,
    Settings,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum OverallSection {
    Raids,
    Mythic,
    Pvp,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ProfileTab {
    Pvp,
    Raids,
    Mythic,
    Endgame,
    Collection,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum ItemQuality {
    Rare,
    Epic,
    Legendary,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Faction {
    Alliance,
    Horde,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum CollectionAvailability {
    Available,
    Restricted,
    Unavailable,
}

impl Faction {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::Alliance => "Alliance",
            Self::Horde => "Horde",
        }
    }

    pub(crate) fn class_name(self) -> &'static str {
        match self {
            Self::Alliance => "alliance",
            Self::Horde => "horde",
        }
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct CharacterProfileData {
    pub(crate) name: String,
    pub(crate) realm: String,
    pub(crate) region: String,
    pub(crate) faction: Faction,
    pub(crate) class_name: String,
    pub(crate) spec: String,
    pub(crate) role: &'static str,
    pub(crate) title: &'static str,
    pub(crate) guild: String,
    pub(crate) hero_talent: &'static str,
    pub(crate) season_summary: &'static str,
    pub(crate) avatar_badge: String,
    pub(crate) avatar_url: Option<String>,
    pub(crate) item_level: u16,
    pub(crate) mythic_score: u32,
    pub(crate) pvp_rating: u16,
    pub(crate) arena_win_rate: u8,
    pub(crate) honor_level: u16,
    pub(crate) honorable_kills: u32,
    pub(crate) weekly_keys: u8,
    pub(crate) mounts_collected: u16,
    pub(crate) pets_collected: u16,
    pub(crate) mounts_availability: CollectionAvailability,
    pub(crate) pets_availability: CollectionAvailability,
    pub(crate) achievements_unlocked: u32,
    pub(crate) achievement_points: u32,
    pub(crate) raid_tier: String,
    pub(crate) raid_progress: String,
    pub(crate) pvp_map_stats: Vec<PvpMapStat>,
    pub(crate) raid_difficulties: Vec<RaidDifficultyProgress>,
    pub(crate) mythic_runs: Vec<MythicRun>,
    pub(crate) gear_slots: Vec<GearSlotData>,
    pub(crate) mount_items: Vec<CollectionItem>,
    pub(crate) pet_items: Vec<CollectionItem>,
    pub(crate) achievements: Vec<AchievementEntry>,
}

impl CharacterProfileData {
    pub(crate) fn initials(&self) -> String {
        let mut letters = self.name.chars();
        match (letters.next(), letters.next()) {
            (Some(first), Some(second)) => {
                format!("{}{}", first.to_ascii_uppercase(), second.to_ascii_uppercase())
            }
            (Some(first), None) => first.to_ascii_uppercase().to_string(),
            _ => String::from("WC"),
        }
    }
}

#[derive(Clone, PartialEq)]
pub(crate) struct GearSlotData {
    pub(crate) slot_name: String,
    pub(crate) label: String,
    pub(crate) item_name: String,
    pub(crate) source: String,
    pub(crate) item_level: u16,
    pub(crate) quality: ItemQuality,
}

#[derive(Clone, PartialEq)]
pub(crate) struct PvpMapStat {
    pub(crate) map: String,
    pub(crate) played: u16,
    pub(crate) won: u16,
    pub(crate) lost: u16,
}

#[derive(Clone, PartialEq)]
pub(crate) struct RaidDifficultyProgress {
    pub(crate) label: String,
    pub(crate) completed: u8,
    pub(crate) total: u8,
    pub(crate) summary: String,
}

#[derive(Clone, PartialEq)]
pub(crate) struct MythicRun {
    pub(crate) dungeon: String,
    pub(crate) level: u8,
    pub(crate) result: String,
    pub(crate) timing: String,
    pub(crate) affix: String,
    pub(crate) icon_url: Option<String>,
}

#[derive(Clone, PartialEq)]
pub(crate) struct CollectionItem {
    pub(crate) label: String,
    pub(crate) name: String,
    pub(crate) source: String,
    pub(crate) collected: bool,
}

#[derive(Clone, PartialEq)]
pub(crate) struct AchievementEntry {
    pub(crate) icon: &'static str,
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) timestamp: String,
}

#[derive(Clone, PartialEq)]
pub(crate) struct FavoriteCharacter {
    pub(crate) route: Route,
    pub(crate) name: &'static str,
    pub(crate) realm: &'static str,
    pub(crate) region: &'static str,
    pub(crate) faction: Faction,
    pub(crate) class_name: &'static str,
    pub(crate) spec: &'static str,
    pub(crate) item_level: u16,
    pub(crate) mythic_score: u32,
    pub(crate) pvp_rating: u16,
}