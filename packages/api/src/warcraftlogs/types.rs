use crate::{ApiError, ApiResult};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeMap;

type WarcraftLogsExtraFields = BTreeMap<String, Value>;

#[derive(Clone, Debug, Default)]
pub struct WarcraftLogsCharacter {
    pub server_region: String,
    pub server_slug: String,
    pub name: String,
}

impl WarcraftLogsCharacter {
    pub fn new(server_region: impl Into<String>, server_slug: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            server_region: server_region.into(),
            server_slug: server_slug.into(),
            name: name.into(),
        }
    }

    pub(super) fn validated(&self) -> ApiResult<ValidatedWarcraftLogsCharacter<'_>> {
        Ok(ValidatedWarcraftLogsCharacter {
            server_region: validate_required_field(&self.server_region, "server region is required")?,
            server_slug: validate_required_field(&self.server_slug, "server slug is required")?,
            name: validate_required_field(&self.name, "character name is required")?,
        })
    }
}

pub(super) struct ValidatedWarcraftLogsCharacter<'a> {
    pub server_region: &'a str,
    pub server_slug: &'a str,
    pub name: &'a str,
}

#[derive(Clone, Debug, Default)]
pub struct WarcraftLogsGuild {
    pub server_region: String,
    pub server_slug: String,
    pub name: String,
}

impl WarcraftLogsGuild {
    pub fn new(server_region: impl Into<String>, server_slug: impl Into<String>, name: impl Into<String>) -> Self {
        Self {
            server_region: server_region.into(),
            server_slug: server_slug.into(),
            name: name.into(),
        }
    }

    pub(super) fn validated(&self) -> ApiResult<ValidatedWarcraftLogsGuild<'_>> {
        Ok(ValidatedWarcraftLogsGuild {
            server_region: validate_required_field(&self.server_region, "server region is required")?,
            server_slug: validate_required_field(&self.server_slug, "server slug is required")?,
            name: validate_required_field(&self.name, "guild name is required")?,
        })
    }
}

pub(super) struct ValidatedWarcraftLogsGuild<'a> {
    pub server_region: &'a str,
    pub server_slug: &'a str,
    pub name: &'a str,
}

#[derive(Clone, Debug, Default)]
pub struct WarcraftLogsCharacterZoneRankingsQuery {
    pub zone_id: Option<u32>,
    pub partition: Option<i32>,
    pub difficulty: Option<u32>,
    pub size: Option<u32>,
    pub metric: Option<String>,
    pub spec_name: Option<String>,
    pub by_bracket: Option<bool>,
    pub timeframe: Option<String>,
    pub compare: Option<String>,
    pub role: Option<String>,
    pub include_private_logs: Option<bool>,
}

impl WarcraftLogsCharacterZoneRankingsQuery {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct WarcraftLogsReportSummaryQuery {
    pub allow_unlisted: Option<bool>,
    pub difficulty: Option<u32>,
    pub encounter_id: Option<u32>,
    pub fight_ids: Vec<u32>,
    pub kill_type: Option<String>,
    pub translate: Option<bool>,
}

impl WarcraftLogsReportSummaryQuery {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct WarcraftLogsReportEventsQuery {
    pub allow_unlisted: Option<bool>,
    pub data_type: Option<String>,
    pub difficulty: Option<u32>,
    pub encounter_id: Option<u32>,
    pub end_time: Option<f64>,
    pub fight_ids: Vec<u32>,
    pub filter_expression: Option<String>,
    pub hostility_type: Option<String>,
    pub kill_type: Option<String>,
    pub limit: Option<u32>,
    pub source_id: Option<u32>,
    pub start_time: Option<f64>,
    pub target_id: Option<u32>,
    pub translate: Option<bool>,
}

impl WarcraftLogsReportEventsQuery {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct WarcraftLogsReportTableQuery {
    pub allow_unlisted: Option<bool>,
    pub data_type: Option<String>,
    pub difficulty: Option<u32>,
    pub encounter_id: Option<u32>,
    pub end_time: Option<f64>,
    pub fight_ids: Vec<u32>,
    pub filter_expression: Option<String>,
    pub hostility_type: Option<String>,
    pub kill_type: Option<String>,
    pub source_id: Option<u32>,
    pub start_time: Option<f64>,
    pub target_id: Option<u32>,
    pub translate: Option<bool>,
}

impl WarcraftLogsReportTableQuery {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct WarcraftLogsGuildZoneRankingQuery {
    pub zone_id: Option<u32>,
    pub difficulty: Option<u32>,
    pub size: Option<u32>,
}

impl WarcraftLogsGuildZoneRankingQuery {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug, Default)]
pub struct WarcraftLogsGuildReportsQuery {
    pub limit: Option<u32>,
    pub page: Option<u32>,
    pub zone_id: Option<u32>,
    pub game_zone_id: Option<u32>,
    pub start_time: Option<f64>,
    pub end_time: Option<f64>,
}

impl WarcraftLogsGuildReportsQuery {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Debug)]
pub struct WarcraftLogsCharacterZoneRankingsResult {
    pub character: WarcraftLogsCharacterSummary,
    pub zone_rankings: Value,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WarcraftLogsCharacterSummary {
    pub id: u32,
    #[serde(rename = "canonicalID")]
    pub canonical_id: u32,
    pub name: String,
    #[serde(rename = "classID")]
    pub class_id: u32,
    pub level: u32,
    pub faction: WarcraftLogsFaction,
    pub server: WarcraftLogsServerSummary,
    #[serde(flatten)]
    pub extra_fields: WarcraftLogsExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WarcraftLogsFaction {
    pub id: u32,
    pub name: String,
    #[serde(flatten)]
    pub extra_fields: WarcraftLogsExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WarcraftLogsReportSummary {
    pub code: String,
    pub title: String,
    pub visibility: String,
    pub start_time: f64,
    pub end_time: f64,
    pub exported_segments: i32,
    pub segments: i32,
    pub revision: i32,
    pub region: Option<WarcraftLogsRegionSummary>,
    pub zone: Option<WarcraftLogsZoneSummary>,
    pub guild: Option<WarcraftLogsGuildSummary>,
    pub fights: Vec<WarcraftLogsReportFightSummary>,
    #[serde(flatten)]
    pub extra_fields: WarcraftLogsExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WarcraftLogsRegionSummary {
    pub id: u32,
    #[serde(rename = "compactName")]
    pub compact_name: String,
    pub name: String,
    pub slug: String,
    #[serde(flatten)]
    pub extra_fields: WarcraftLogsExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WarcraftLogsServerSummary {
    pub id: u32,
    pub name: String,
    pub slug: String,
    #[serde(rename = "normalizedName")]
    pub normalized_name: String,
    pub region: WarcraftLogsRegionSummary,
    #[serde(flatten)]
    pub extra_fields: WarcraftLogsExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WarcraftLogsZoneSummary {
    pub id: u32,
    pub name: String,
    #[serde(flatten)]
    pub extra_fields: WarcraftLogsExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WarcraftLogsGuildSummary {
    pub id: u32,
    pub name: String,
    pub server: WarcraftLogsServerSummary,
    #[serde(flatten)]
    pub extra_fields: WarcraftLogsExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WarcraftLogsReportFightSummary {
    pub id: u32,
    #[serde(rename = "encounterID")]
    pub encounter_id: u32,
    pub name: String,
    pub kill: Option<bool>,
    pub difficulty: Option<u32>,
    pub size: Option<u32>,
    pub start_time: f64,
    pub end_time: f64,
    pub boss_percentage: Option<f64>,
    pub fight_percentage: Option<f64>,
    pub average_item_level: Option<f64>,
    #[serde(flatten)]
    pub extra_fields: WarcraftLogsExtraFields,
}

#[derive(Clone, Debug)]
pub struct WarcraftLogsReportEventPage {
    pub report_code: String,
    pub report_title: String,
    pub data: Option<Value>,
    pub next_page_timestamp: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct WarcraftLogsReportTableResult {
    pub report_code: String,
    pub report_title: String,
    pub table: Option<Value>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WarcraftLogsGuildProfile {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub faction: WarcraftLogsFaction,
    pub server: WarcraftLogsServerSummary,
    #[serde(rename = "competitionMode")]
    pub competition_mode: bool,
    #[serde(rename = "stealthMode")]
    pub stealth_mode: bool,
    #[serde(rename = "type")]
    pub guild_type: i32,
    pub tags: Option<Vec<WarcraftLogsGuildTag>>,
    #[serde(flatten)]
    pub extra_fields: WarcraftLogsExtraFields,
}

#[derive(Clone, Debug)]
pub struct WarcraftLogsGuildZoneRankingResult {
    pub guild: WarcraftLogsGuildProfile,
    pub zone_ranking: WarcraftLogsGuildZoneRankings,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WarcraftLogsGuildZoneRankings {
    pub progress: Option<WarcraftLogsWorldRegionServerRanks>,
    pub speed: Option<WarcraftLogsWorldRegionServerRanks>,
    #[serde(rename = "completeRaidSpeed")]
    pub complete_raid_speed: Option<WarcraftLogsWorldRegionServerRanks>,
    #[serde(flatten)]
    pub extra_fields: WarcraftLogsExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WarcraftLogsWorldRegionServerRanks {
    #[serde(rename = "worldRank")]
    pub world_rank: Option<WarcraftLogsRank>,
    #[serde(rename = "regionRank")]
    pub region_rank: Option<WarcraftLogsRank>,
    #[serde(rename = "serverRank")]
    pub server_rank: Option<WarcraftLogsRank>,
    #[serde(flatten)]
    pub extra_fields: WarcraftLogsExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WarcraftLogsRank {
    pub number: u32,
    pub percentile: Option<u32>,
    pub color: String,
    #[serde(flatten)]
    pub extra_fields: WarcraftLogsExtraFields,
}

#[derive(Clone, Debug)]
pub struct WarcraftLogsGuildReportsPage {
    pub reports: Vec<WarcraftLogsReportListItem>,
    pub pagination: WarcraftLogsPaginationMeta,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WarcraftLogsReportListItem {
    pub code: String,
    pub title: String,
    pub visibility: String,
    pub start_time: f64,
    pub end_time: f64,
    pub zone: Option<WarcraftLogsZoneSummary>,
    #[serde(flatten)]
    pub extra_fields: WarcraftLogsExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WarcraftLogsPaginationMeta {
    pub total: u32,
    pub per_page: u32,
    pub current_page: u32,
    pub from: Option<u32>,
    pub to: Option<u32>,
    pub last_page: u32,
    pub has_more_pages: bool,
    #[serde(flatten)]
    pub extra_fields: WarcraftLogsExtraFields,
}

#[derive(Clone, Debug, Deserialize)]
pub struct WarcraftLogsGuildTag {
    pub id: u32,
    pub name: String,
    #[serde(flatten)]
    pub extra_fields: WarcraftLogsExtraFields,
}

fn validate_required_field<'a>(value: &'a str, error_message: &'static str) -> ApiResult<&'a str> {
    let trimmed = value.trim();

    if trimmed.is_empty() {
        return Err(ApiError::InvalidInput(error_message));
    }

    Ok(trimmed)
}