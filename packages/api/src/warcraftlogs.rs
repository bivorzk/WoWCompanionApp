#[path = "warcraftlogs/types.rs"]
mod types;

use crate::{ApiConfig, ApiError, ApiResult};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{env, sync::OnceLock};

pub use self::types::*;

const WARCRAFT_LOGS_CLIENT_ID_ENV: &str = "WCL_CLIENT_ID";
const WARCRAFT_LOGS_CLIENT_SECRET_ENV: &str = "WCL_CLIENT_SECRET";
const WARCRAFT_LOGS_OAUTH_URL: &str = "https://www.warcraftlogs.com/oauth/token";
const WARCRAFT_LOGS_PUBLIC_GRAPHQL_URL: &str = "https://www.warcraftlogs.com/api/v2/client";

const CHARACTER_ZONE_RANKINGS_QUERY: &str = r#"
query CharacterZoneRankings(
    $name: String!
    $serverSlug: String!
    $serverRegion: String!
    $zoneId: Int
    $partition: Int
    $difficulty: Int
    $size: Int
    $metric: CharacterPageRankingMetricType
    $specName: String
    $byBracket: Boolean
    $timeframe: RankingTimeframeType
    $compare: RankingCompareType
    $role: RoleType
    $includePrivateLogs: Boolean
) {
    characterData {
        character(name: $name, serverSlug: $serverSlug, serverRegion: $serverRegion) {
            id
            canonicalID
            name
            classID
            level
            faction {
                id
                name
            }
            server {
                id
                name
                slug
                normalizedName
                region {
                    id
                    compactName
                    name
                    slug
                }
            }
            zoneRankings(
                zoneID: $zoneId
                partition: $partition
                difficulty: $difficulty
                size: $size
                metric: $metric
                specName: $specName
                byBracket: $byBracket
                timeframe: $timeframe
                compare: $compare
                role: $role
                includePrivateLogs: $includePrivateLogs
            )
        }
    }
}
"#;

const REPORT_SUMMARY_QUERY: &str = r#"
query ReportSummary(
    $code: String!
    $allowUnlisted: Boolean
    $difficulty: Int
    $encounterId: Int
    $fightIds: [Int!]
    $killType: KillType
    $translate: Boolean
) {
    reportData {
        report(code: $code, allowUnlisted: $allowUnlisted) {
            code
            title
            visibility
            startTime
            endTime
            exportedSegments
            segments
            revision
            region {
                id
                compactName
                name
                slug
            }
            zone {
                id
                name
            }
            guild {
                id
                name
                server {
                    id
                    name
                    slug
                    normalizedName
                    region {
                        id
                        compactName
                        name
                        slug
                    }
                }
            }
            fights(difficulty: $difficulty, encounterID: $encounterId, fightIDs: $fightIds, killType: $killType, translate: $translate) {
                id
                encounterID
                name
                kill
                difficulty
                size
                startTime
                endTime
                bossPercentage
                fightPercentage
                averageItemLevel
            }
        }
    }
}
"#;

const REPORT_EVENTS_QUERY: &str = r#"
query ReportEvents(
    $code: String!
    $allowUnlisted: Boolean
    $dataType: EventDataType
    $difficulty: Int
    $encounterId: Int
    $endTime: Float
    $fightIds: [Int!]
    $filterExpression: String
    $hostilityType: HostilityType
    $killType: KillType
    $limit: Int
    $sourceId: Int
    $startTime: Float
    $targetId: Int
    $translate: Boolean
) {
    reportData {
        report(code: $code, allowUnlisted: $allowUnlisted) {
            code
            title
            events(
                dataType: $dataType
                difficulty: $difficulty
                encounterID: $encounterId
                endTime: $endTime
                fightIDs: $fightIds
                filterExpression: $filterExpression
                hostilityType: $hostilityType
                killType: $killType
                limit: $limit
                sourceID: $sourceId
                startTime: $startTime
                targetID: $targetId
                translate: $translate
            ) {
                data
                nextPageTimestamp
            }
        }
    }
}
"#;

const REPORT_TABLE_QUERY: &str = r#"
query ReportTable(
    $code: String!
    $allowUnlisted: Boolean
    $dataType: TableDataType
    $difficulty: Int
    $encounterId: Int
    $endTime: Float
    $fightIds: [Int!]
    $filterExpression: String
    $hostilityType: HostilityType
    $killType: KillType
    $sourceId: Int
    $startTime: Float
    $targetId: Int
    $translate: Boolean
) {
    reportData {
        report(code: $code, allowUnlisted: $allowUnlisted) {
            code
            title
            table(
                dataType: $dataType
                difficulty: $difficulty
                encounterID: $encounterId
                endTime: $endTime
                fightIDs: $fightIds
                filterExpression: $filterExpression
                hostilityType: $hostilityType
                killType: $killType
                sourceID: $sourceId
                startTime: $startTime
                targetID: $targetId
                translate: $translate
            )
        }
    }
}
"#;

const GUILD_PROFILE_QUERY: &str = r#"
query GuildProfile($name: String!, $serverSlug: String!, $serverRegion: String!) {
    guildData {
        guild(name: $name, serverSlug: $serverSlug, serverRegion: $serverRegion) {
            id
            name
            description
            competitionMode
            stealthMode
            type
            faction {
                id
                name
            }
            server {
                id
                name
                slug
                normalizedName
                region {
                    id
                    compactName
                    name
                    slug
                }
            }
            tags {
                id
                name
            }
        }
    }
}
"#;

const GUILD_ZONE_RANKING_QUERY: &str = r#"
query GuildZoneRanking(
    $name: String!
    $serverSlug: String!
    $serverRegion: String!
    $zoneId: Int
    $difficulty: Int
    $size: Int
) {
    guildData {
        guild(name: $name, serverSlug: $serverSlug, serverRegion: $serverRegion) {
            id
            name
            description
            competitionMode
            stealthMode
            type
            faction {
                id
                name
            }
            server {
                id
                name
                slug
                normalizedName
                region {
                    id
                    compactName
                    name
                    slug
                }
            }
            tags {
                id
                name
            }
            zoneRanking(zoneId: $zoneId) {
                progress(size: $size) {
                    worldRank {
                        number
                        percentile
                        color
                    }
                    regionRank {
                        number
                        percentile
                        color
                    }
                    serverRank {
                        number
                        percentile
                        color
                    }
                }
                speed(size: $size, difficulty: $difficulty) {
                    worldRank {
                        number
                        percentile
                        color
                    }
                    regionRank {
                        number
                        percentile
                        color
                    }
                    serverRank {
                        number
                        percentile
                        color
                    }
                }
                completeRaidSpeed(size: $size, difficulty: $difficulty) {
                    worldRank {
                        number
                        percentile
                        color
                    }
                    regionRank {
                        number
                        percentile
                        color
                    }
                    serverRank {
                        number
                        percentile
                        color
                    }
                }
            }
        }
    }
}
"#;

const GUILD_REPORTS_QUERY: &str = r#"
query GuildReports(
    $guildName: String!
    $guildServerSlug: String!
    $guildServerRegion: String!
    $limit: Int
    $page: Int
    $zoneId: Int
    $gameZoneId: Int
    $startTime: Float
    $endTime: Float
) {
    reportData {
        reports(
            guildName: $guildName
            guildServerSlug: $guildServerSlug
            guildServerRegion: $guildServerRegion
            limit: $limit
            page: $page
            zoneID: $zoneId
            gameZoneID: $gameZoneId
            startTime: $startTime
            endTime: $endTime
        ) {
            data {
                code
                title
                visibility
                startTime
                endTime
                zone {
                    id
                    name
                }
            }
            total
            per_page
            current_page
            from
            to
            last_page
            has_more_pages
        }
    }
}
"#;

static DOTENV_LOADED: OnceLock<()> = OnceLock::new();

#[derive(Clone, Debug)]
pub struct WarcraftLogsClient {
        http: reqwest::Client,
}

impl Default for WarcraftLogsClient {
        fn default() -> Self {
                Self::new()
        }
}

impl WarcraftLogsClient {
        pub fn new() -> Self {
                Self {
                        http: reqwest::Client::new(),
                }
        }

        pub async fn authenticate(&self, config: &ApiConfig) -> ApiResult<()> {
                self.fetch_access_token(config).await.map(|_| ())
        }

        pub async fn fetch_character_zone_rankings(&self, config: &ApiConfig, character: &WarcraftLogsCharacter, query: &WarcraftLogsCharacterZoneRankingsQuery) -> ApiResult<WarcraftLogsCharacterZoneRankingsResult> {
                let character = character.validated()?;
                let server_region = normalize_region(character.server_region)?;
                let server_slug = normalize_server_slug(character.server_slug)?;
                let response: CharacterZoneRankingsResponse = self
                        .post_graphql(
                                config,
                                CHARACTER_ZONE_RANKINGS_QUERY,
                                CharacterZoneRankingsVariables::new(character.name, server_slug.as_str(), server_region.as_str(), query),
                        )
                        .await?;
                let character = require_data(response.character_data.character, "character was missing from the Warcraft Logs response")?;

                Ok(WarcraftLogsCharacterZoneRankingsResult {
                        character: WarcraftLogsCharacterSummary {
                                id: character.id,
                                canonical_id: character.canonical_id,
                                name: character.name,
                                class_id: character.class_id,
                                level: character.level,
                                faction: character.faction,
                                server: character.server,
                                extra_fields: character.extra_fields,
                        },
                        zone_rankings: character.zone_rankings.unwrap_or(Value::Null),
                })
        }

        pub async fn fetch_report_summary(&self, config: &ApiConfig, report_code: &str, query: &WarcraftLogsReportSummaryQuery) -> ApiResult<WarcraftLogsReportSummary> {
                let report_code = validate_required_field(report_code, "report code is required")?;
                let response: ReportSummaryResponse = self.post_graphql(config, REPORT_SUMMARY_QUERY, ReportSummaryVariables::new(report_code, query)).await?;

                require_data(response.report_data.report, format!("report '{report_code}' was missing from the Warcraft Logs response"))
        }

        pub async fn fetch_report_events(&self, config: &ApiConfig, report_code: &str, query: &WarcraftLogsReportEventsQuery) -> ApiResult<WarcraftLogsReportEventPage> {
                let report_code = validate_required_field(report_code, "report code is required")?;
                let response: ReportEventsResponse = self.post_graphql(config, REPORT_EVENTS_QUERY, ReportEventsVariables::new(report_code, query)).await?;
                let report = require_data(response.report_data.report, format!("report '{report_code}' was missing from the Warcraft Logs response"))?;
                let (data, next_page_timestamp) = match report.events {
                        Some(events) => (events.data, events.next_page_timestamp),
                        None => (None, None),
                };

                Ok(WarcraftLogsReportEventPage {
                        report_code: report.code,
                        report_title: report.title,
                        data,
                        next_page_timestamp,
                })
        }

        pub async fn fetch_report_table(&self, config: &ApiConfig, report_code: &str, query: &WarcraftLogsReportTableQuery) -> ApiResult<WarcraftLogsReportTableResult> {
                let report_code = validate_required_field(report_code, "report code is required")?;
                let response: ReportTableResponse = self.post_graphql(config, REPORT_TABLE_QUERY, ReportTableVariables::new(report_code, query)).await?;
                let report = require_data(response.report_data.report, format!("report '{report_code}' was missing from the Warcraft Logs response"))?;

                Ok(WarcraftLogsReportTableResult {
                        report_code: report.code,
                        report_title: report.title,
                        table: report.table,
                })
        }

        pub async fn fetch_guild_profile(&self, config: &ApiConfig, guild: &WarcraftLogsGuild) -> ApiResult<WarcraftLogsGuildProfile> {
                let guild = guild.validated()?;
                let server_region = normalize_region(guild.server_region)?;
                let server_slug = normalize_server_slug(guild.server_slug)?;
                let response: GuildProfileResponse = self
                        .post_graphql(
                                config,
                                GUILD_PROFILE_QUERY,
                                GuildLookupVariables::new(guild.name, server_slug.as_str(), server_region.as_str()),
                        )
                        .await?;

                require_data(response.guild_data.guild, format!("guild '{}' was missing from the Warcraft Logs response", guild.name))
        }

        pub async fn fetch_guild_zone_ranking(&self, config: &ApiConfig, guild: &WarcraftLogsGuild, query: &WarcraftLogsGuildZoneRankingQuery) -> ApiResult<WarcraftLogsGuildZoneRankingResult> {
                let guild = guild.validated()?;
                let server_region = normalize_region(guild.server_region)?;
                let server_slug = normalize_server_slug(guild.server_slug)?;
                let response: GuildZoneRankingResponse = self
                        .post_graphql(
                                config,
                                GUILD_ZONE_RANKING_QUERY,
                                GuildZoneRankingVariables::new(guild.name, server_slug.as_str(), server_region.as_str(), query),
                        )
                        .await?;
                let guild = require_data(response.guild_data.guild, format!("guild '{}' was missing from the Warcraft Logs response", guild.name))?;
                let zone_ranking = require_data(guild.zone_ranking, "guild zone ranking was missing from the Warcraft Logs response")?;

                Ok(WarcraftLogsGuildZoneRankingResult {
                        guild: WarcraftLogsGuildProfile {
                                id: guild.id,
                                name: guild.name,
                                description: guild.description,
                                faction: guild.faction,
                                server: guild.server,
                                competition_mode: guild.competition_mode,
                                stealth_mode: guild.stealth_mode,
                                guild_type: guild.guild_type,
                                tags: guild.tags,
                                extra_fields: guild.extra_fields,
                        },
                        zone_ranking,
                })
        }

        pub async fn fetch_guild_reports(&self, config: &ApiConfig, guild: &WarcraftLogsGuild, query: &WarcraftLogsGuildReportsQuery) -> ApiResult<WarcraftLogsGuildReportsPage> {
                let guild = guild.validated()?;
                let server_region = normalize_region(guild.server_region)?;
                let server_slug = normalize_server_slug(guild.server_slug)?;
                let response: GuildReportsResponse = self
                        .post_graphql(
                                config,
                                GUILD_REPORTS_QUERY,
                                GuildReportsVariables::new(guild.name, server_slug.as_str(), server_region.as_str(), query),
                        )
                        .await?;
                let reports = response.report_data.reports;

                Ok(WarcraftLogsGuildReportsPage {
                        reports: reports.data.unwrap_or_default(),
                        pagination: WarcraftLogsPaginationMeta {
                                total: reports.total,
                                per_page: reports.per_page,
                                current_page: reports.current_page,
                                from: reports.from,
                                to: reports.to,
                                last_page: reports.last_page,
                                has_more_pages: reports.has_more_pages,
                                extra_fields: reports.extra_fields,
                        },
                })
        }

        async fn post_graphql<T, V>(&self, config: &ApiConfig, query: &str, variables: V) -> ApiResult<T>
        where
                T: DeserializeOwned,
                V: Serialize,
        {
                let token = self.fetch_access_token(config).await?;
                let response = self
                        .http
                        .post(WARCRAFT_LOGS_PUBLIC_GRAPHQL_URL)
                        .bearer_auth(token.access_token)
                        .json(&WarcraftLogsGraphQlRequest { query, variables })
                        .send()
                        .await?
                        .error_for_status()?
                        .json::<WarcraftLogsGraphQlResponse<T>>()
                        .await?;

                if let Some(errors) = response.errors {
                        let message = errors
                                .into_iter()
                                .map(|error| error.message)
                                .collect::<Vec<_>>()
                                .join(" | ");

                        return Err(ApiError::GraphQl(message));
                }

                require_data(response.data, "Warcraft Logs GraphQL response did not include data")
        }

        async fn fetch_access_token(&self, config: &ApiConfig) -> ApiResult<WarcraftLogsAccessToken> {
                let credentials = resolve_warcraft_logs_credentials(config)?;
                let token = self
                        .http
                        .post(WARCRAFT_LOGS_OAUTH_URL)
                        .basic_auth(credentials.client_id, Some(credentials.client_secret))
                        .form(&[("grant_type", "client_credentials")])
                        .send()
                        .await?
                        .error_for_status()?
                        .json::<WarcraftLogsAccessToken>()
                        .await?;

                Ok(token)
        }
}

#[derive(Serialize)]
struct WarcraftLogsGraphQlRequest<'a, V> {
        query: &'a str,
        variables: V,
}

#[derive(Deserialize)]
struct WarcraftLogsGraphQlResponse<T> {
        data: Option<T>,
        errors: Option<Vec<WarcraftLogsGraphQlError>>,
}

#[derive(Deserialize)]
struct WarcraftLogsGraphQlError {
        message: String,
}

#[derive(Deserialize)]
struct WarcraftLogsAccessToken {
        access_token: String,
}

#[derive(Clone, Debug)]
struct WarcraftLogsCredentials {
        client_id: String,
        client_secret: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CharacterZoneRankingsVariables<'a> {
        name: &'a str,
        server_slug: &'a str,
        server_region: &'a str,
        zone_id: Option<u32>,
        partition: Option<i32>,
        difficulty: Option<u32>,
        size: Option<u32>,
        metric: Option<&'a str>,
        spec_name: Option<&'a str>,
        by_bracket: Option<bool>,
        timeframe: Option<&'a str>,
        compare: Option<&'a str>,
        role: Option<&'a str>,
        include_private_logs: Option<bool>,
}

impl<'a> CharacterZoneRankingsVariables<'a> {
        fn new(name: &'a str, server_slug: &'a str, server_region: &'a str, query: &'a WarcraftLogsCharacterZoneRankingsQuery) -> Self {
                Self {
                        name,
                        server_slug,
                        server_region,
                        zone_id: query.zone_id,
                        partition: query.partition,
                        difficulty: query.difficulty,
                        size: query.size,
                        metric: normalize_optional_str(query.metric.as_deref()),
                        spec_name: normalize_optional_str(query.spec_name.as_deref()),
                        by_bracket: query.by_bracket,
                        timeframe: normalize_optional_str(query.timeframe.as_deref()),
                        compare: normalize_optional_str(query.compare.as_deref()),
                        role: normalize_optional_str(query.role.as_deref()),
                        include_private_logs: query.include_private_logs,
                }
        }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ReportSummaryVariables<'a> {
        code: &'a str,
        allow_unlisted: Option<bool>,
        difficulty: Option<u32>,
        encounter_id: Option<u32>,
        fight_ids: Option<Vec<u32>>,
        kill_type: Option<&'a str>,
        translate: Option<bool>,
}

impl<'a> ReportSummaryVariables<'a> {
        fn new(code: &'a str, query: &'a WarcraftLogsReportSummaryQuery) -> Self {
                Self {
                        code,
                        allow_unlisted: query.allow_unlisted,
                        difficulty: query.difficulty,
                        encounter_id: query.encounter_id,
                        fight_ids: (!query.fight_ids.is_empty()).then_some(query.fight_ids.clone()),
                        kill_type: normalize_optional_str(query.kill_type.as_deref()),
                        translate: query.translate,
                }
        }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ReportEventsVariables<'a> {
        code: &'a str,
        allow_unlisted: Option<bool>,
        data_type: Option<&'a str>,
        difficulty: Option<u32>,
        encounter_id: Option<u32>,
        end_time: Option<f64>,
        fight_ids: Option<Vec<u32>>,
        filter_expression: Option<&'a str>,
        hostility_type: Option<&'a str>,
        kill_type: Option<&'a str>,
        limit: Option<u32>,
        source_id: Option<u32>,
        start_time: Option<f64>,
        target_id: Option<u32>,
        translate: Option<bool>,
}

impl<'a> ReportEventsVariables<'a> {
        fn new(code: &'a str, query: &'a WarcraftLogsReportEventsQuery) -> Self {
                Self {
                        code,
                        allow_unlisted: query.allow_unlisted,
                        data_type: normalize_optional_str(query.data_type.as_deref()),
                        difficulty: query.difficulty,
                        encounter_id: query.encounter_id,
                        end_time: query.end_time,
                        fight_ids: (!query.fight_ids.is_empty()).then_some(query.fight_ids.clone()),
                        filter_expression: normalize_optional_str(query.filter_expression.as_deref()),
                        hostility_type: normalize_optional_str(query.hostility_type.as_deref()),
                        kill_type: normalize_optional_str(query.kill_type.as_deref()),
                        limit: query.limit,
                        source_id: query.source_id,
                        start_time: query.start_time,
                        target_id: query.target_id,
                        translate: query.translate,
                }
        }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct ReportTableVariables<'a> {
        code: &'a str,
        allow_unlisted: Option<bool>,
        data_type: Option<&'a str>,
        difficulty: Option<u32>,
        encounter_id: Option<u32>,
        end_time: Option<f64>,
        fight_ids: Option<Vec<u32>>,
        filter_expression: Option<&'a str>,
        hostility_type: Option<&'a str>,
        kill_type: Option<&'a str>,
        source_id: Option<u32>,
        start_time: Option<f64>,
        target_id: Option<u32>,
        translate: Option<bool>,
}

impl<'a> ReportTableVariables<'a> {
        fn new(code: &'a str, query: &'a WarcraftLogsReportTableQuery) -> Self {
                Self {
                        code,
                        allow_unlisted: query.allow_unlisted,
                        data_type: normalize_optional_str(query.data_type.as_deref()),
                        difficulty: query.difficulty,
                        encounter_id: query.encounter_id,
                        end_time: query.end_time,
                        fight_ids: (!query.fight_ids.is_empty()).then_some(query.fight_ids.clone()),
                        filter_expression: normalize_optional_str(query.filter_expression.as_deref()),
                        hostility_type: normalize_optional_str(query.hostility_type.as_deref()),
                        kill_type: normalize_optional_str(query.kill_type.as_deref()),
                        source_id: query.source_id,
                        start_time: query.start_time,
                        target_id: query.target_id,
                        translate: query.translate,
                }
        }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GuildLookupVariables<'a> {
        name: &'a str,
        server_slug: &'a str,
        server_region: &'a str,
}

impl<'a> GuildLookupVariables<'a> {
        fn new(name: &'a str, server_slug: &'a str, server_region: &'a str) -> Self {
                Self {
                        name,
                        server_slug,
                        server_region,
                }
        }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GuildZoneRankingVariables<'a> {
        name: &'a str,
        server_slug: &'a str,
        server_region: &'a str,
        zone_id: Option<u32>,
        difficulty: Option<u32>,
        size: Option<u32>,
}

impl<'a> GuildZoneRankingVariables<'a> {
        fn new(name: &'a str, server_slug: &'a str, server_region: &'a str, query: &'a WarcraftLogsGuildZoneRankingQuery) -> Self {
                Self {
                        name,
                        server_slug,
                        server_region,
                        zone_id: query.zone_id,
                        difficulty: query.difficulty,
                        size: query.size,
                }
        }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GuildReportsVariables<'a> {
        guild_name: &'a str,
        guild_server_slug: &'a str,
        guild_server_region: &'a str,
        limit: Option<u32>,
        page: Option<u32>,
        zone_id: Option<u32>,
        game_zone_id: Option<u32>,
        start_time: Option<f64>,
        end_time: Option<f64>,
}

impl<'a> GuildReportsVariables<'a> {
        fn new(guild_name: &'a str, guild_server_slug: &'a str, guild_server_region: &'a str, query: &'a WarcraftLogsGuildReportsQuery) -> Self {
                Self {
                        guild_name,
                        guild_server_slug,
                        guild_server_region,
                        limit: query.limit,
                        page: query.page,
                        zone_id: query.zone_id,
                        game_zone_id: query.game_zone_id,
                        start_time: query.start_time,
                        end_time: query.end_time,
                }
        }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CharacterZoneRankingsResponse {
        character_data: CharacterZoneRankingsNodeData,
}

#[derive(Deserialize)]
struct CharacterZoneRankingsNodeData {
        character: Option<CharacterZoneRankingsNode>,
}

#[derive(Deserialize)]
struct CharacterZoneRankingsNode {
        id: u32,
        #[serde(rename = "canonicalID")]
        canonical_id: u32,
        name: String,
        #[serde(rename = "classID")]
        class_id: u32,
        level: u32,
        faction: WarcraftLogsFaction,
        server: WarcraftLogsServerSummary,
        #[serde(rename = "zoneRankings")]
        zone_rankings: Option<Value>,
        #[serde(flatten)]
        extra_fields: std::collections::BTreeMap<String, Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReportSummaryResponse {
        report_data: ReportSummaryNodeData,
}

#[derive(Deserialize)]
struct ReportSummaryNodeData {
        report: Option<WarcraftLogsReportSummary>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReportEventsResponse {
        report_data: ReportEventsNodeData,
}

#[derive(Deserialize)]
struct ReportEventsNodeData {
        report: Option<ReportEventsNode>,
}

#[derive(Deserialize)]
struct ReportEventsNode {
        code: String,
        title: String,
        events: Option<ReportEventPaginator>,
}

#[derive(Deserialize)]
struct ReportEventPaginator {
        data: Option<Value>,
        #[serde(rename = "nextPageTimestamp")]
        next_page_timestamp: Option<f64>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ReportTableResponse {
        report_data: ReportTableNodeData,
}

#[derive(Deserialize)]
struct ReportTableNodeData {
        report: Option<ReportTableNode>,
}

#[derive(Deserialize)]
struct ReportTableNode {
        code: String,
        title: String,
        table: Option<Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GuildProfileResponse {
        guild_data: GuildProfileNodeData,
}

#[derive(Deserialize)]
struct GuildProfileNodeData {
        guild: Option<WarcraftLogsGuildProfile>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GuildZoneRankingResponse {
        guild_data: GuildZoneRankingNodeData,
}

#[derive(Deserialize)]
struct GuildZoneRankingNodeData {
        guild: Option<GuildZoneRankingNode>,
}

#[derive(Deserialize)]
struct GuildZoneRankingNode {
        id: u32,
        name: String,
        description: String,
        faction: WarcraftLogsFaction,
        server: WarcraftLogsServerSummary,
        #[serde(rename = "competitionMode")]
        competition_mode: bool,
        #[serde(rename = "stealthMode")]
        stealth_mode: bool,
        #[serde(rename = "type")]
        guild_type: i32,
        tags: Option<Vec<WarcraftLogsGuildTag>>,
        #[serde(rename = "zoneRanking")]
        zone_ranking: Option<WarcraftLogsGuildZoneRankings>,
        #[serde(flatten)]
        extra_fields: std::collections::BTreeMap<String, Value>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GuildReportsResponse {
        report_data: GuildReportsNodeData,
}

#[derive(Deserialize)]
struct GuildReportsNodeData {
        reports: GuildReportsPaginationNode,
}

#[derive(Deserialize)]
struct GuildReportsPaginationNode {
        data: Option<Vec<WarcraftLogsReportListItem>>,
        total: u32,
        per_page: u32,
        current_page: u32,
        from: Option<u32>,
        to: Option<u32>,
        last_page: u32,
        has_more_pages: bool,
        #[serde(flatten)]
        extra_fields: std::collections::BTreeMap<String, Value>,
}

fn resolve_warcraft_logs_credentials(config: &ApiConfig) -> ApiResult<WarcraftLogsCredentials> {
        load_dotenv();

        Ok(WarcraftLogsCredentials {
                client_id: resolve_config_value(&config.wcl_client_id, WARCRAFT_LOGS_CLIENT_ID_ENV, "Warcraft Logs client ID is required")?,
                client_secret: resolve_config_value(&config.wcl_client_secret, WARCRAFT_LOGS_CLIENT_SECRET_ENV, "Warcraft Logs client secret is required")?,
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
        Ok(validate_required_field(region, "server region is required")?.to_ascii_lowercase())
}

fn normalize_server_slug(server_slug: &str) -> ApiResult<String> {
        Ok(validate_required_field(server_slug, "server slug is required")?.to_ascii_lowercase())
}

fn normalize_optional_str(value: Option<&str>) -> Option<&str> {
        value.and_then(read_non_empty)
}

fn validate_required_field<'a>(value: &'a str, error_message: &'static str) -> ApiResult<&'a str> {
        read_non_empty(value).ok_or(ApiError::InvalidInput(error_message))
}

fn read_non_empty(value: &str) -> Option<&str> {
        let trimmed = value.trim();

        (!trimmed.is_empty()).then_some(trimmed)
}

fn require_data<T>(value: Option<T>, message: impl Into<String>) -> ApiResult<T> {
        value.ok_or(ApiError::MissingData(message.into()))
}

fn load_dotenv() {
        DOTENV_LOADED.get_or_init(|| {
                let _ = dotenvy::dotenv();
        });
}