use crate::{ApiError, ApiResult};
use serde::{Deserialize, Deserializer};
use serde_json::Value;
use std::collections::BTreeMap;

type RaiderIoExtraFields = BTreeMap<String, Value>;

#[derive(Clone, Debug, Deserialize)]
pub struct RaiderIoCharacterProfile {
    pub name: String,
    pub realm: String,
    pub region: String,
    #[serde(rename = "class")]
    pub class_name: Option<String>,
    pub active_spec_name: Option<String>,
    pub thumbnail_url: Option<String>,
    pub achievement_points: Option<u32>,
    #[serde(default)]
    pub mythic_plus_recent_runs: Vec<RaiderIoKeystoneRun>,
    #[serde(default, deserialize_with = "deserialize_single_or_vec")]
    pub mythic_plus_scores_by_season: Vec<RaiderIoPublicMythicPlusScoresBySeason>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct RaiderIoPublicMythicPlusScoresBySeason {
    pub season: String,
    pub scores: RaiderIoPublicRoleScores,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct RaiderIoPublicRoleScores {
    #[serde(default)]
    pub all: f64,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
enum RaiderIoOneOrMany<T> {
    One(T),
    Many(Vec<T>),
}

fn deserialize_single_or_vec<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    match RaiderIoOneOrMany::<T>::deserialize(deserializer)? {
        RaiderIoOneOrMany::One(value) => Ok(vec![value]),
        RaiderIoOneOrMany::Many(values) => Ok(values),
    }
}

#[derive(Clone, Debug, Default)]
pub struct RaiderIoCharacter {
    pub region: String,
    pub realm: String,
    pub name: String,
}

impl RaiderIoCharacter {
    pub fn new(
        region: impl Into<String>,
        realm: impl Into<String>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            region: region.into(),
            realm: realm.into(),
            name: name.into(),
        }
    }

    pub(super) fn validated(&self) -> ApiResult<ValidatedRaiderIoCharacter<'_>> {
        Ok(ValidatedRaiderIoCharacter {
            region: validate_required_field(&self.region, "region is required")?,
            realm: validate_required_field(&self.realm, "realm is required")?,
            name: validate_required_field(&self.name, "name is required")?,
        })
    }
}

#[derive(Clone, Debug, Default)]
pub struct RaiderIoCharacterOverviewQuery {
    pub season: Option<String>,
    pub tier: Option<u32>,
}

impl RaiderIoCharacterOverviewQuery {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn season(mut self, season: impl Into<String>) -> Self {
        self.season = Some(season.into());
        self
    }

    pub fn tier(mut self, tier: u32) -> Self {
        self.tier = Some(tier);
        self
    }

    pub(super) fn to_query_pairs(&self) -> ApiResult<Vec<(&'static str, String)>> {
        let mut query_pairs = Vec::new();

        if let Some(season) = self.season.as_deref() {
            let season = validate_required_field(season, "season cannot be empty")?;
            query_pairs.push(("season", season.to_string()));
        }

        if let Some(tier) = self.tier {
            query_pairs.push(("tier", tier.to_string()));
        }

        Ok(query_pairs)
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaiderIoCharacterOverview {
    pub character_raid_progress: Option<RaiderIoCharacterRaidProgress>,
    pub character_mythic_plus_progress: Option<Value>,
    pub character_details: RaiderIoCharacterDetails,
    #[serde(flatten)]
    pub extra_sections: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaiderIoCharacterRaidProgress {
    pub tier: String,
    #[serde(default)]
    pub raid_progress: Vec<RaiderIoRaidProgressEntry>,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaiderIoRaidProgressEntry {
    pub raid: Option<String>,
    pub summary: Option<String>,
    pub expansion_id: Option<u32>,
    pub total_bosses: Option<u32>,
    pub normal_bosses_killed: Option<u32>,
    pub heroic_bosses_killed: Option<u32>,
    pub mythic_bosses_killed: Option<u32>,
    #[serde(default)]
    pub encounters_defeated: RaiderIoRaidEncounterProgress,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaiderIoRaidEncounterProgress {
    #[serde(default)]
    pub normal: Vec<RaiderIoRaidEncounterKill>,
    #[serde(default)]
    pub heroic: Vec<RaiderIoRaidEncounterKill>,
    #[serde(default)]
    pub mythic: Vec<RaiderIoRaidEncounterKill>,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaiderIoRaidEncounterKill {
    pub slug: Option<String>,
    pub first_defeated: Option<String>,
    pub last_defeated: Option<String>,
    pub num_kills: Option<u32>,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaiderIoCharacterMythicPlusProgress {
    pub season_slug: String,
    pub season: String,
    #[serde(default)]
    pub mythic_plus_scores: RaiderIoMythicPlusScoreBuckets,
    pub best_mythic_plus_score: Option<RaiderIoMythicPlusSeasonScore>,
    #[serde(default)]
    pub keystone_aggregate_stats: Vec<RaiderIoKeystoneAggregateStat>,
    pub previous_mythic_plus_score: Option<RaiderIoMythicPlusSeasonScore>,
    #[serde(default)]
    pub mythic_plus_ranks: RaiderIoMythicPlusRankBuckets,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct RaiderIoMythicPlusScoreBuckets {
    #[serde(flatten)]
    pub buckets: BTreeMap<String, RaiderIoMythicPlusScoreBucket>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaiderIoMythicPlusScoreBucket {
    pub score: f64,
    pub score_color: String,
    #[serde(default)]
    pub runs: Vec<RaiderIoKeystoneRun>,
    #[serde(default)]
    pub alternate_runs: Vec<RaiderIoKeystoneRun>,
    #[serde(default)]
    pub raw_runs: Vec<RaiderIoKeystoneRun>,
    #[serde(default)]
    pub raw_alternate_runs: Vec<RaiderIoKeystoneRun>,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct RaiderIoMythicPlusRankBuckets {
    #[serde(flatten)]
    pub buckets: BTreeMap<String, RaiderIoRanks>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct RaiderIoRanks {
    pub world: u32,
    pub region: u32,
    pub realm: u32,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct RaiderIoKeystoneAggregateStat {
    pub level: u32,
    pub count: u32,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaiderIoMythicPlusSeasonScore {
    pub score: f64,
    pub score_color: String,
    pub season: RaiderIoSeasonSummary,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RaiderIoSeasonSummary {
    pub slug: String,
    pub name: String,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RaiderIoKeystoneRun {
    pub dungeon: String,
    #[serde(alias = "shortName")]
    pub short_name: String,
    #[serde(alias = "mythicLevel")]
    pub mythic_level: u32,
    #[serde(alias = "keystoneRunId")]
    pub keystone_run_id: u64,
    #[serde(alias = "completedAt")]
    pub completed_at: String,
    #[serde(alias = "clearTimeMs")]
    pub clear_time_ms: u64,
    #[serde(alias = "parTimeMs")]
    pub par_time_ms: u64,
    #[serde(alias = "numKeystoneUpgrades")]
    pub num_keystone_upgrades: u32,
    #[serde(alias = "mapChallengeModeId")]
    pub map_challenge_mode_id: u32,
    #[serde(alias = "zoneId")]
    pub zone_id: u32,
    #[serde(alias = "zoneExpansionId")]
    pub zone_expansion_id: u32,
    #[serde(alias = "iconUrl")]
    pub icon_url: String,
    #[serde(alias = "backgroundImageUrl")]
    pub background_image_url: String,
    pub score: f64,
    pub url: String,
    #[serde(default)]
    pub affixes: Vec<RaiderIoAffix>,
    pub spec: Option<RaiderIoRunSpec>,
    pub role: Option<String>,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RaiderIoAffix {
    pub id: u32,
    pub name: String,
    pub description: Option<String>,
    pub icon: String,
    pub slug: Option<String>,
    pub icon_url: Option<String>,
    pub wowhead_url: Option<String>,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RaiderIoRunSpec {
    pub id: u64,
    pub name: String,
    pub slug: String,
    pub role: Option<String>,
    pub class_id: Option<u32>,
    pub is_melee: Option<bool>,
    pub ordinal: Option<u32>,
    pub patch: Option<String>,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaiderIoCharacterDetails {
    pub character: RaiderIoDetailedCharacter,
    pub team: Option<Value>,
    pub raid_progress: Option<Value>,
    pub item_details: Option<Value>,
    pub tier: Option<Value>,
    pub meta: Option<Value>,
    #[serde(flatten)]
    pub extra_sections: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaiderIoDetailedCharacter {
    pub id: u64,
    #[serde(rename = "persona_id")]
    pub persona_id: Option<u64>,
    pub name: String,
    pub level: u32,
    pub achievement_points: Option<u32>,
    pub item_level_equipped: Option<f64>,
    pub thumbnail_url: Option<String>,
    pub class: Option<RaiderIoNamedEntity>,
    pub race: Option<RaiderIoNamedEntity>,
    pub faction: Option<String>,
    pub spec: Option<RaiderIoNamedEntity>,
    pub realm: Option<RaiderIoRealmSummary>,
    pub region: Option<RaiderIoRegionSummary>,
    pub guild: Option<Value>,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RaiderIoNamedEntity {
    pub id: u64,
    pub name: String,
    pub slug: Option<String>,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RaiderIoRealmSummary {
    pub id: Option<u64>,
    pub connected_realm_id: Option<u64>,
    pub wow_realm_id: Option<u64>,
    pub wow_connected_realm_id: Option<u64>,
    pub name: String,
    pub alt_name: Option<String>,
    pub slug: String,
    pub alt_slug: Option<String>,
    pub locale: Option<String>,
    pub is_connected: Option<bool>,
    pub realm_type: Option<String>,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct RaiderIoRegionSummary {
    pub name: String,
    pub slug: String,
    #[serde(rename = "short_name")]
    pub short_name: Option<String>,
    #[serde(flatten)]
    pub extra_fields: RaiderIoExtraFields,
}

pub(super) struct ValidatedRaiderIoCharacter<'a> {
    pub(super) region: &'a str,
    pub(super) realm: &'a str,
    pub(super) name: &'a str,
}

fn validate_required_field<'a>(value: &'a str, message: &'static str) -> ApiResult<&'a str> {
    let value = value.trim();

    if value.is_empty() {
        return Err(ApiError::InvalidInput(message));
    }

    Ok(value)
}