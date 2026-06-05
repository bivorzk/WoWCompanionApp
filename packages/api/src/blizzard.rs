#[path = "blizzard_wow/types.rs"]
mod types;

use crate::{ApiConfig, ApiError, ApiResult};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::{env, sync::OnceLock};

pub use self::types::*;

const BLIZZARD_CLIENT_ID_ENV: &str = "BLIZZARD_CLIENT_ID";
const BLIZZARD_CLIENT_SECRET_ENV: &str = "BLIZZARD_CLIENT_SECRET";
const BLIZZARD_OAUTH_BASE_URL: &str = "https://oauth.battle.net";
const DEFAULT_BLIZZARD_LOCALE: &str = "en_US";

static DOTENV_LOADED: OnceLock<()> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct BlizzardClient {
    http: reqwest::Client,
}

impl Default for BlizzardClient {
    fn default() -> Self {
        Self::new()
    }
}

impl BlizzardClient {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
        }
    }

    pub async fn authenticate(&self, config: &ApiConfig) -> ApiResult<()> {
        self.fetch_access_token(config).await.map(|_| ())
    }

    pub async fn fetch_pvp_tiers(&self, config: &ApiConfig, region: &str) -> ApiResult<PvPTiers> {
        self.get_static_json(config, region, &["data", "wow", "pvp-tier", "index"], DEFAULT_BLIZZARD_LOCALE).await
    }

    pub async fn fetch_pvp_tier_media(&self, config: &ApiConfig, region: &str, pvp_tier_id: u32) -> ApiResult<PvPTierMedia> {
        let pvp_tier_id = validate_identifier(pvp_tier_id, "PvP tier id must be greater than zero")?;
        let pvp_tier_id = pvp_tier_id.to_string();

        self.get_static_json(config, region, &["data", "wow", "media", "pvp-tier", pvp_tier_id.as_str()], DEFAULT_BLIZZARD_LOCALE)
            .await
    }

    pub async fn fetch_character_appearance(&self, config: &ApiConfig, region: &str, realm_slug: &str, character_name: &str) -> ApiResult<CharacterAppearance> {
        let realm_slug = normalize_profile_path_segment(realm_slug, "realm slug is required")?;
        let character_name = normalize_profile_path_segment(character_name, "character name is required")?;

        self.get_profile_json(
            config,
            region,
            &[
                "profile",
                "wow",
                "character",
                realm_slug.as_str(),
                character_name.as_str(),
                "appearance",
            ],
            DEFAULT_BLIZZARD_LOCALE,
        )
        .await
    }

    pub async fn fetch_character_equipment(&self, config: &ApiConfig, region: &str, realm_slug: &str, character_name: &str) -> ApiResult<CharacterEquipmentSummary> {
        let realm_slug = normalize_profile_path_segment(realm_slug, "realm slug is required")?;
        let character_name = normalize_profile_path_segment(character_name, "character name is required")?;

        self.get_profile_json(
            config,
            region,
            &[
                "profile",
                "wow",
                "character",
                realm_slug.as_str(),
                character_name.as_str(),
                "equipment",
            ],
            DEFAULT_BLIZZARD_LOCALE,
        )
        .await
    }

    pub async fn fetch_character_achievements(&self, config: &ApiConfig, region: &str, realm_slug: &str, character_name: &str) -> ApiResult<CharacterAchievementsSummary> {
        let realm_slug = normalize_profile_path_segment(realm_slug, "realm slug is required")?;
        let character_name = normalize_profile_path_segment(character_name, "character name is required")?;

        self.get_profile_json(
            config,
            region,
            &[
                "profile",
                "wow",
                "character",
                realm_slug.as_str(),
                character_name.as_str(),
                "achievements",
            ],
            DEFAULT_BLIZZARD_LOCALE,
        )
        .await
    }

    pub async fn fetch_character_mounts_collection(&self, config: &ApiConfig, region: &str, realm_slug: &str, character_name: &str) -> ApiResult<CharacterMountsCollection> {
        let realm_slug = normalize_profile_path_segment(realm_slug, "realm slug is required")?;
        let character_name = normalize_profile_path_segment(character_name, "character name is required")?;

        self.get_profile_json(
            config,
            region,
            &[
                "profile",
                "wow",
                "character",
                realm_slug.as_str(),
                character_name.as_str(),
                "collections",
                "mounts",
            ],
            DEFAULT_BLIZZARD_LOCALE,
        )
        .await
    }

    pub async fn fetch_character_pets_collection(&self, config: &ApiConfig, region: &str, realm_slug: &str, character_name: &str) -> ApiResult<CharacterPetsCollection> {
        let realm_slug = normalize_profile_path_segment(realm_slug, "realm slug is required")?;
        let character_name = normalize_profile_path_segment(character_name, "character name is required")?;

        self.get_profile_json(
            config,
            region,
            &[
                "profile",
                "wow",
                "character",
                realm_slug.as_str(),
                character_name.as_str(),
                "collections",
                "pets",
            ],
            DEFAULT_BLIZZARD_LOCALE,
        )
        .await
    }

    pub async fn fetch_character_pvp_summary(&self, config: &ApiConfig, region: &str, realm_slug: &str, character_name: &str) -> ApiResult<CharacterPvPSummary> {
        let realm_slug = normalize_profile_path_segment(realm_slug, "realm slug is required")?;
        let character_name = normalize_profile_path_segment(character_name, "character name is required")?;

        self.get_profile_json(
            config,
            region,
            &[
                "profile",
                "wow",
                "character",
                realm_slug.as_str(),
                character_name.as_str(),
                "pvp-summary",
            ],
            DEFAULT_BLIZZARD_LOCALE,
        )
        .await
    }

    pub async fn fetch_character_pvp_bracket_statistics(&self, config: &ApiConfig, region: &str, realm_slug: &str, character_name: &str, bracket: &str) -> ApiResult<CharacterPvPBracketStatistics> {
        let realm_slug = normalize_profile_path_segment(realm_slug, "realm slug is required")?;
        let character_name = normalize_profile_path_segment(character_name, "character name is required")?;
        let bracket = normalize_profile_path_segment(bracket, "PvP bracket is required")?;

        self.get_profile_json(
            config,
            region,
            &[
                "profile",
                "wow",
                "character",
                realm_slug.as_str(),
                character_name.as_str(),
                "pvp-bracket",
                bracket.as_str(),
            ],
            DEFAULT_BLIZZARD_LOCALE,
        )
        .await
    }

    pub async fn fetch_item_details(&self, config: &ApiConfig, region: &str, item_id: u32) -> ApiResult<ItemDetails> {
        let item_id = validate_identifier(item_id, "item id must be greater than zero")?;
        let item_id = item_id.to_string();

        self.get_static_json(config, region, &["data", "wow", "item", item_id.as_str()], DEFAULT_BLIZZARD_LOCALE).await
    }

    async fn get_static_json<T>(&self, config: &ApiConfig, region: &str, path_segments: &[&str], locale: &str) -> ApiResult<T>
    where
        T: DeserializeOwned,
    {
        let region = normalize_region(region)?;
        let namespace = format!("static-{region}");

        self.get_json(config, &region, path_segments, &namespace, locale).await
    }

    async fn get_profile_json<T>(&self, config: &ApiConfig, region: &str, path_segments: &[&str], locale: &str) -> ApiResult<T>
    where
        T: DeserializeOwned,
    {
        let region = normalize_region(region)?;
        let namespace = format!("profile-{region}");

        self.get_json(config, &region, path_segments, &namespace, locale).await
    }

    async fn get_json<T>(&self, config: &ApiConfig, region: &str, path_segments: &[&str], namespace: &str, locale: &str) -> ApiResult<T>
    where
        T: DeserializeOwned,
    {
        let token = self.fetch_access_token(config).await?;
        let url = build_blizzard_api_url(region, path_segments, namespace, locale)?;

        let response = self
            .http
            .get(url)
            .bearer_auth(token.access_token)
            .send()
            .await?
            .error_for_status()?
            .json::<T>()
            .await?;

        Ok(response)
    }

    async fn fetch_access_token(&self, config: &ApiConfig) -> ApiResult<BlizzardAccessToken> {
        let credentials = resolve_blizzard_credentials(config)?;
        let url = format!("{BLIZZARD_OAUTH_BASE_URL}/token");

        let token = self
            .http
            .post(url)
            .basic_auth(credentials.client_id, Some(credentials.client_secret))
            .form(&[("grant_type", "client_credentials")])
            .send()
            .await?
            .error_for_status()?
            .json::<BlizzardAccessToken>()
            .await?;

        Ok(token)
    }
}

#[derive(Clone, Debug)]
struct BlizzardCredentials {
    client_id: String,
    client_secret: String,
}

#[derive(Clone, Debug, Deserialize)]
struct BlizzardAccessToken {
    access_token: String,
}

fn build_blizzard_api_url(region: &str, path_segments: &[&str], namespace: &str, locale: &str) -> ApiResult<reqwest::Url> {
    let mut url = reqwest::Url::parse(&format!("https://{region}.api.blizzard.com"))
        .map_err(|_| ApiError::InvalidInput("Blizzard region produced an invalid API URL"))?;

    {
        let mut segments = url
            .path_segments_mut()
            .map_err(|_| ApiError::InvalidInput("Blizzard API base URL should support path segments"))?;

        for path_segment in path_segments {
            segments.push(path_segment);
        }
    }

    {
        let locale = validate_required_field(locale, "locale is required")?;
        let mut url_query_pairs = url.query_pairs_mut();

        url_query_pairs.append_pair("namespace", namespace);
        url_query_pairs.append_pair("locale", locale);
    }

    Ok(url)
}

fn resolve_blizzard_credentials(config: &ApiConfig) -> ApiResult<BlizzardCredentials> {
    load_dotenv();

    Ok(BlizzardCredentials {
        client_id: resolve_config_value(&config.blizzard_client_id, BLIZZARD_CLIENT_ID_ENV, "Blizzard client ID is required")?,
        client_secret: resolve_config_value(&config.blizzard_client_secret, BLIZZARD_CLIENT_SECRET_ENV, "Blizzard client secret is required")?,
    })
}

fn resolve_config_value(config_value: &str, env_key: &str, error_message: &'static str) -> ApiResult<String> {
    if let Some(value) = read_non_empty(config_value) {
        return Ok(value.to_string());
    }

    if let Ok(value) = env::var(env_key) {
        if let Some(value) = read_non_empty(&value) {
            return Ok(value.to_string());
        }
    }

    Err(ApiError::InvalidInput(error_message))
}

fn normalize_region(region: &str) -> ApiResult<String> {
    Ok(validate_required_field(region, "Blizzard region is required")?.to_ascii_lowercase())
}

fn normalize_profile_path_segment(value: &str, error_message: &'static str) -> ApiResult<String> {
    Ok(validate_required_field(value, error_message)?.to_ascii_lowercase())
}

fn validate_identifier(identifier: u32, error_message: &'static str) -> ApiResult<u32> {
    if identifier == 0 {
        return Err(ApiError::InvalidInput(error_message));
    }

    Ok(identifier)
}

fn validate_required_field<'a>(value: &'a str, error_message: &'static str) -> ApiResult<&'a str> {
    read_non_empty(value).ok_or(ApiError::InvalidInput(error_message))
}

fn read_non_empty(value: &str) -> Option<&str> {
    let trimmed = value.trim();

    (!trimmed.is_empty()).then_some(trimmed)
}

fn load_dotenv() {
    DOTENV_LOADED.get_or_init(|| {
        let _ = dotenvy::dotenv();
    });
}