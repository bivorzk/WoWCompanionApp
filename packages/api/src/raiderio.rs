mod types;

use crate::{ApiError, ApiResult};
use serde::de::DeserializeOwned;
use std::{env, sync::OnceLock};

pub use self::types::*;

const DEFAULT_RAIDER_IO_PUBLIC_API_BASE_URL: &str = "https://raider.io/api/v1";
const DEFAULT_RAIDER_IO_SITE_API_BASE_URL: &str = "https://raider.io/api";
const RAIDER_IO_PUBLIC_API_BASE_URL_ENV: &str = "RAIDER_IO_PUBLIC_API_BASE_URL";
const RAIDER_IO_SITE_API_BASE_URL_ENV: &str = "RAIDER_IO_SITE_API_BASE_URL";

static DOTENV_LOADED: OnceLock<()> = OnceLock::new();

#[derive(Clone, Debug)]
struct RaiderIoBaseUrls {
    public_api: String,
    site_api: String,
}

#[derive(Clone, Debug)]
pub struct RaiderIoClient {
    http: reqwest::Client,
    base_urls: RaiderIoBaseUrls,
}

impl Default for RaiderIoClient {
    fn default() -> Self {
        Self::new()
    }
}

impl RaiderIoClient {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
            base_urls: load_raider_io_base_urls(),
        }
    }

    pub async fn fetch_character_profile(&self, region: &str, realm: &str, name: &str) -> ApiResult<RaiderIoCharacterProfile> {
        let character = RaiderIoCharacter::new(region, realm, name);
        self.fetch_character_profile_by_ref(&character).await
    }

    pub async fn fetch_character_profile_by_ref(&self, character: &RaiderIoCharacter) -> ApiResult<RaiderIoCharacterProfile> {
        let character = character.validated()?;
        let query_pairs = vec![
            ("region", character.region.to_string()),
            ("realm", character.realm.to_string()),
            ("name", character.name.to_string()),
        ];

        self.get_public_api_json(&["characters", "profile"], &query_pairs)
            .await
    }

    /// Fetch the richer site payload used by Raider.IO's character pages.
    ///
    /// Example path:
    /// `/api/characters/eu/ravencrest/Bvb?season=season-mn-1&tier=35`
    pub async fn fetch_character_overview(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery) -> ApiResult<RaiderIoCharacterOverview> {
        let character = character.validated()?;
        let query_pairs = query.to_query_pairs()?;

        self.get_site_api_json(&["characters", character.region, character.realm, character.name], &query_pairs).await
    }

    pub async fn fetch_character_raid_progress(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery) -> ApiResult<RaiderIoCharacterRaidProgress> {
        self.fetch_character_overview_projection(character, query, |overview| {
            require_data(overview.character_raid_progress, "character raid progress was missing from the overview payload")
        })
        .await
    }

    pub async fn fetch_character_raid_progress_entries(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery) -> ApiResult<Vec<RaiderIoRaidProgressEntry>> {
        self.fetch_character_raid_progress(character, query).await.map(|raid_progress| raid_progress.raid_progress)
    }

    pub async fn fetch_character_mythic_plus_progress(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery) -> ApiResult<RaiderIoCharacterMythicPlusProgress> {
        self.fetch_character_overview_projection(character, query, |overview| {
            require_data(overview.character_mythic_plus_progress, "character Mythic+ progress was missing from the overview payload")
        })
        .await
    }

    pub async fn fetch_character_mythic_plus_score_buckets(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery) -> ApiResult<RaiderIoMythicPlusScoreBuckets> {
        self.fetch_character_mythic_plus_progress(character, query).await.map(|mythic_plus_progress| mythic_plus_progress.mythic_plus_scores)
    }

    pub async fn fetch_character_mythic_plus_score_bucket(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery, bucket_key: &str) -> ApiResult<RaiderIoMythicPlusScoreBucket> {
        let bucket_key = validate_bucket_key(bucket_key)?;
        let mut score_buckets = self.fetch_character_mythic_plus_score_buckets(character, query).await?;

        require_data(score_buckets.buckets.remove(bucket_key), format!("Mythic+ score bucket '{bucket_key}' was missing"))
    }

    pub async fn fetch_character_mythic_plus_rank_buckets(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery) -> ApiResult<RaiderIoMythicPlusRankBuckets> {
        self.fetch_character_mythic_plus_progress(character, query).await.map(|mythic_plus_progress| mythic_plus_progress.mythic_plus_ranks)
    }

    pub async fn fetch_character_mythic_plus_rank_bucket(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery, bucket_key: &str) -> ApiResult<RaiderIoRanks> {
        let bucket_key = validate_bucket_key(bucket_key)?;
        let mut rank_buckets = self.fetch_character_mythic_plus_rank_buckets(character, query).await?;

        require_data(rank_buckets.buckets.remove(bucket_key), format!("Mythic+ rank bucket '{bucket_key}' was missing"))
    }

    pub async fn fetch_character_keystone_aggregate_stats(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery) -> ApiResult<Vec<RaiderIoKeystoneAggregateStat>> {
        self.fetch_character_mythic_plus_progress(character, query).await.map(|mythic_plus_progress| mythic_plus_progress.keystone_aggregate_stats)
    }

    pub async fn fetch_character_previous_mythic_plus_score(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery) -> ApiResult<RaiderIoMythicPlusSeasonScore> {
        self.fetch_character_mythic_plus_progress(character, query).await.and_then(|mythic_plus_progress| {
            require_data(mythic_plus_progress.previous_mythic_plus_score, "previous Mythic+ score was missing from the overview payload")
        })
    }

    pub async fn fetch_character_previous_mythic_plus_season(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery) -> ApiResult<RaiderIoSeasonSummary> {
        self.fetch_character_previous_mythic_plus_score(character, query).await.map(|previous_score| previous_score.season)
    }

    pub async fn fetch_character_runs_from_score_bucket(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery, bucket_key: &str) -> ApiResult<Vec<RaiderIoKeystoneRun>> {
        let score_bucket = self.fetch_character_mythic_plus_score_bucket(character, query, bucket_key).await?;
        let mut runs = score_bucket.runs;

        runs.extend(score_bucket.alternate_runs);
        runs.extend(score_bucket.raw_runs);
        runs.extend(score_bucket.raw_alternate_runs);

        Ok(runs)
    }

    pub async fn fetch_character_affixes_from_score_bucket(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery, bucket_key: &str) -> ApiResult<Vec<RaiderIoAffix>> {
        self.fetch_character_runs_from_score_bucket(character, query, bucket_key).await.map(|runs| {
            runs.into_iter().flat_map(|run| run.affixes.into_iter()).collect()
        })
    }

    pub async fn fetch_character_run_specs_from_score_bucket(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery, bucket_key: &str) -> ApiResult<Vec<RaiderIoRunSpec>> {
        self.fetch_character_runs_from_score_bucket(character, query, bucket_key).await.map(|runs| runs.into_iter().filter_map(|run| run.spec).collect())
    }

    pub async fn fetch_character_details(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery) -> ApiResult<RaiderIoCharacterDetails> {
        self.fetch_character_overview_projection(character, query, |overview| Ok(overview.character_details)).await
    }

    pub async fn fetch_detailed_character(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery) -> ApiResult<RaiderIoDetailedCharacter> {
        self.fetch_character_details(character, query).await.map(|details| details.character)
    }

    pub async fn fetch_character_class_entity(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery) -> ApiResult<RaiderIoNamedEntity> {
        self.fetch_detailed_character(character, query).await.and_then(|detailed_character| {
            require_data(detailed_character.class, "character class entity was missing from the detailed character payload")
        })
    }

    pub async fn fetch_character_race_entity(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery) -> ApiResult<RaiderIoNamedEntity> {
        self.fetch_detailed_character(character, query).await.and_then(|detailed_character| {
            require_data(detailed_character.race, "character race entity was missing from the detailed character payload")
        })
    }

    pub async fn fetch_character_spec_entity(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery) -> ApiResult<RaiderIoNamedEntity> {
        self.fetch_detailed_character(character, query).await.and_then(|detailed_character| {
            require_data(detailed_character.spec, "character spec entity was missing from the detailed character payload")
        })
    }

    pub async fn fetch_character_realm_summary(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery) -> ApiResult<RaiderIoRealmSummary> {
        self.fetch_detailed_character(character, query).await.and_then(|detailed_character| {
            require_data(detailed_character.realm, "character realm summary was missing from the detailed character payload")
        })
    }

    pub async fn fetch_character_region_summary(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery) -> ApiResult<RaiderIoRegionSummary> {
        self.fetch_detailed_character(character, query).await.and_then(|detailed_character| {
            require_data(detailed_character.region, "character region summary was missing from the detailed character payload")
        })
    }

    async fn get_public_api_json<T>(&self, path_segments: &[&str], query_pairs: &[(&str, String)]) -> ApiResult<T>
    where
        T: DeserializeOwned,
    {
        self.get_json(&self.base_urls.public_api, path_segments, query_pairs).await
    }

    async fn get_site_api_json<T>(&self, path_segments: &[&str], query_pairs: &[(&str, String)]) -> ApiResult<T>
    where
        T: DeserializeOwned,
    {
        self.get_json(&self.base_urls.site_api, path_segments, query_pairs).await
    }

    async fn fetch_character_overview_projection<T, F>(&self, character: &RaiderIoCharacter, query: &RaiderIoCharacterOverviewQuery, projection: F) -> ApiResult<T>
    where
        F: FnOnce(RaiderIoCharacterOverview) -> ApiResult<T>,
    {
        projection(self.fetch_character_overview(character, query).await?)
    }

    async fn get_json<T>(&self, base_url: &str, path_segments: &[&str], query_pairs: &[(&str, String)]) -> ApiResult<T>
    where
        T: DeserializeOwned,
    {
        let url = build_raider_io_url(base_url, path_segments, query_pairs);

        let response = self
            .http
            .get(url)
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await?;

        Ok(response)
    }
}

fn build_raider_io_url(base_url: &str, path_segments: &[&str], query_pairs: &[(&str, String)]) -> reqwest::Url {
    let mut url = reqwest::Url::parse(base_url).expect("Raider.IO base URL should be valid");

    {
        let mut segments = url
            .path_segments_mut()
            .expect("Raider.IO base URL should support path segments");

        for path_segment in path_segments {
            segments.push(path_segment);
        }
    }

    {
        let mut url_query_pairs = url.query_pairs_mut();

        for (key, value) in query_pairs {
            url_query_pairs.append_pair(key, value);
        }
    }

    url
}

fn load_raider_io_base_urls() -> RaiderIoBaseUrls {
    DOTENV_LOADED.get_or_init(|| {
        let _ = dotenvy::dotenv();
    });

    RaiderIoBaseUrls {
        public_api: read_env_or_default(RAIDER_IO_PUBLIC_API_BASE_URL_ENV, DEFAULT_RAIDER_IO_PUBLIC_API_BASE_URL),
        site_api: read_env_or_default(RAIDER_IO_SITE_API_BASE_URL_ENV, DEFAULT_RAIDER_IO_SITE_API_BASE_URL),
    }
}

fn read_env_or_default(key: &str, default: &str) -> String {
    env::var(key)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| default.to_string())
}

fn require_data<T>(value: Option<T>, message: impl Into<String>) -> ApiResult<T> {
    value.ok_or_else(|| ApiError::MissingData(message.into()))
}

fn validate_bucket_key<'a>(bucket_key: &'a str) -> ApiResult<&'a str> {
    let bucket_key = bucket_key.trim();

    if bucket_key.is_empty() {
        return Err(ApiError::InvalidInput("bucket key is required"));
    }

    Ok(bucket_key)
}