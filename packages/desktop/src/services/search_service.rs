use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{types::FavoriteCharacter, Route};

const SEARCH_HISTORY_LIMIT: usize = 20;
const EU_REALMS: &[&str] = &[
    "Ravencrest",
    "Tarren Mill",
    "Silvermoon",
    "Kazzak",
    "Draenor",
    "Twisting Nether",
    "Stormscale",
    "Blackmoore",
    "Hyjal",
    "Outland",
    "Archimonde",
    "Burning Legion",
];
const US_REALMS: &[&str] = &[
    "Area 52",
    "Stormrage",
    "Illidan",
    "Mal'Ganis",
    "Proudmoore",
    "Sargeras",
    "Thrall",
    "Tichondrius",
    "Zul'jin",
    "Emerald Dream",
    "Kel'Thuzad",
    "Frostmourne",
];

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct SearchHistoryEntry {
    pub(crate) name: String,
    pub(crate) realm_name: String,
    pub(crate) realm_slug: String,
    pub(crate) region: String,
}

impl SearchHistoryEntry {
    pub(crate) fn from_favorite(favorite: &FavoriteCharacter) -> Self {
        Self {
            name: favorite.name.to_string(),
            realm_name: favorite.realm.to_string(),
            realm_slug: slugify_segment(favorite.realm),
            region: normalize_region(favorite.region).unwrap_or_else(|| String::from("eu")),
        }
    }

    pub(crate) fn to_route(&self) -> Route {
        Route::CharacterProfile {
            region: normalize_region(&self.region).unwrap_or_else(|| String::from("eu")),
            realm: self.realm_slug.clone(),
            name: slugify_segment(&self.name),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct RealmHint {
    pub(crate) realm_name: String,
    pub(crate) realm_slug: String,
    pub(crate) region: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum SearchMessageKind {
    Hint,
    Error,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SearchMessage {
    pub(crate) kind: SearchMessageKind,
    pub(crate) text: String,
}

impl SearchMessage {
    pub(crate) fn hint(text: impl Into<String>) -> Self {
        Self {
            kind: SearchMessageKind::Hint,
            text: text.into(),
        }
    }

    pub(crate) fn error(text: impl Into<String>) -> Self {
        Self {
            kind: SearchMessageKind::Error,
            text: text.into(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct SearchDropdownData {
    pub(crate) recent: Vec<SearchHistoryEntry>,
    pub(crate) favorites: Vec<SearchHistoryEntry>,
    pub(crate) realm_hints: Vec<RealmHint>,
}

impl SearchDropdownData {
    pub(crate) fn is_empty(&self) -> bool {
        self.recent.is_empty() && self.favorites.is_empty() && self.realm_hints.is_empty()
    }
}

pub(crate) fn parse_search(input: &str, region: &str) -> Option<SearchHistoryEntry> {
    let (name, realm_raw) = input.split_once('-')?;
    let name = collapse_spaces(name);
    let realm_slug = slugify_segment(realm_raw);

    if name.is_empty() || realm_slug.is_empty() {
        return None;
    }

    Some(SearchHistoryEntry {
        name,
        realm_name: display_realm(&realm_slug),
        realm_slug,
        region: normalize_region(region).unwrap_or_else(|| String::from("eu")),
    })
}

pub(crate) fn search_message(query: &str) -> Option<SearchMessage> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return None;
    }

    if trimmed.contains('/') || trimmed.contains(',') {
        return Some(SearchMessage::error("Use CharacterName-Realm"));
    }

    let Some((name, realm)) = trimmed.split_once('-') else {
        let name_hint = collapse_spaces(trimmed);
        if name_hint.is_empty() {
            return None;
        }

        return Some(SearchMessage::hint(format!(
            "Add a realm - {}-RealmName",
            name_hint
        )));
    };

    if name.trim().is_empty() {
        return Some(SearchMessage::error("Character name is missing"));
    }

    if realm.trim().is_empty() {
        return Some(SearchMessage::error("Realm is missing"));
    }

    None
}

pub(crate) fn load_search_history() -> Vec<SearchHistoryEntry> {
    let path = history_path();
    let Ok(contents) = fs::read_to_string(path) else {
        return Vec::new();
    };

    serde_json::from_str(&contents).unwrap_or_default()
}

pub(crate) fn push_search_history(entry: SearchHistoryEntry) -> Vec<SearchHistoryEntry> {
    let mut entries = load_search_history();
    entries.retain(|current| !same_character(current, &entry));
    entries.insert(0, entry);
    entries.truncate(SEARCH_HISTORY_LIMIT);
    save_search_history(&entries);
    entries
}

pub(crate) fn build_search_dropdown(
    query: &str,
    region: &str,
    recent: &[SearchHistoryEntry],
    favorites: &[FavoriteCharacter],
) -> SearchDropdownData {
    let favorites = favorites
        .iter()
        .map(SearchHistoryEntry::from_favorite)
        .filter(|entry| matches_search_filter(entry, query, region))
        .take(5)
        .collect::<Vec<_>>();
    let recent = recent
        .iter()
        .filter(|entry| matches_search_filter(entry, query, region))
        .filter(|entry| !favorites.iter().any(|favorite| same_character(favorite, entry)))
        .take(5)
        .cloned()
        .collect::<Vec<_>>();
    let realm_hints = realm_hints(query, region, recent.as_slice(), favorites.as_slice());

    SearchDropdownData {
        recent,
        favorites,
        realm_hints,
    }
}

fn realm_hints(
    query: &str,
    region: &str,
    recent: &[SearchHistoryEntry],
    favorites: &[SearchHistoryEntry],
) -> Vec<RealmHint> {
    let Some((name, realm_fragment)) = query.split_once('-') else {
        return Vec::new();
    };

    if collapse_spaces(name).is_empty() {
        return Vec::new();
    }

    let realm_fragment = lookup_key(realm_fragment);
    if realm_fragment.len() < 2 {
        return Vec::new();
    }

    let mut hints = realm_catalog(region, recent, favorites)
        .into_iter()
        .filter(|hint| lookup_key(&hint.realm_name).contains(&realm_fragment))
        .collect::<Vec<_>>();
    hints.truncate(6);
    hints
}

fn realm_catalog(
    region: &str,
    recent: &[SearchHistoryEntry],
    favorites: &[SearchHistoryEntry],
) -> Vec<RealmHint> {
    let region = normalize_region(region).unwrap_or_else(|| String::from("eu"));
    let mut hints = Vec::new();

    for realm in base_realms(&region) {
        push_realm_hint(&mut hints, realm.to_string(), region.clone());
    }

    for entry in recent.iter().chain(favorites.iter()) {
        if normalize_region(&entry.region).as_deref() == Some(region.as_str()) {
            push_realm_hint(&mut hints, entry.realm_name.clone(), region.clone());
        }
    }

    hints
}

fn push_realm_hint(hints: &mut Vec<RealmHint>, realm_name: String, region: String) {
    let realm_slug = slugify_segment(&realm_name);
    if realm_slug.is_empty() {
        return;
    }

    if hints.iter().any(|hint| hint.realm_slug == realm_slug && hint.region == region) {
        return;
    }

    hints.push(RealmHint {
        realm_name,
        realm_slug,
        region,
    });
}

fn matches_search_filter(entry: &SearchHistoryEntry, query: &str, region: &str) -> bool {
    let selected_region = normalize_region(region).unwrap_or_else(|| String::from("eu"));
    if normalize_region(&entry.region).as_deref() != Some(selected_region.as_str()) {
        return false;
    }

    let trimmed = query.trim();
    if trimmed.is_empty() {
        return true;
    }

    let (name_query, realm_query) = match trimmed.split_once('-') {
        Some((name, realm)) => (lookup_key(name), lookup_key(realm)),
        None => (lookup_key(trimmed), String::new()),
    };
    let entry_name = lookup_key(&entry.name);
    let entry_realm = lookup_key(&entry.realm_name);

    (name_query.is_empty() || entry_name.contains(&name_query))
        && (realm_query.is_empty() || entry_realm.contains(&realm_query))
}

fn history_path() -> PathBuf {
    dirs::data_local_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("WowCompanion")
        .join("history.json")
}

fn save_search_history(entries: &[SearchHistoryEntry]) {
    let path = history_path();
    let Some(parent) = path.parent() else {
        return;
    };

    if fs::create_dir_all(parent).is_err() {
        return;
    }

    let Ok(contents) = serde_json::to_string_pretty(entries) else {
        return;
    };

    let _ = fs::write(path, contents);
}

fn same_character(left: &SearchHistoryEntry, right: &SearchHistoryEntry) -> bool {
    lookup_key(&left.name) == lookup_key(&right.name)
        && left.realm_slug == right.realm_slug
        && normalize_region(&left.region) == normalize_region(&right.region)
}

fn collapse_spaces(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn display_realm(value: &str) -> String {
    value.split('-')
        .filter(|segment| !segment.is_empty())
        .map(capitalize_segment)
        .collect::<Vec<_>>()
        .join(" ")
}

fn capitalize_segment(value: &str) -> String {
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

fn lookup_key(value: &str) -> String {
    value
        .chars()
        .filter_map(|character| match character {
            'a'..='z' | '0'..='9' => Some(character),
            'A'..='Z' => Some(character.to_ascii_lowercase()),
            _ => None,
        })
        .collect()
}

fn base_realms(region: &str) -> &'static [&'static str] {
    match normalize_region(region).as_deref() {
        Some("us") => US_REALMS,
        _ => EU_REALMS,
    }
}

fn normalize_region(value: &str) -> Option<String> {
    match value.trim().to_ascii_lowercase().as_str() {
        "eu" | "europe" => Some(String::from("eu")),
        "us" | "americas" => Some(String::from("us")),
        "kr" | "korea" => Some(String::from("kr")),
        "tw" | "taiwan" => Some(String::from("tw")),
        _ => None,
    }
}

fn slugify_segment(value: &str) -> String {
    let mut slug = String::new();
    let mut previous_was_dash = false;

    for character in value.trim().chars() {
        match character {
            'a'..='z' | '0'..='9' => {
                slug.push(character);
                previous_was_dash = false;
            }
            'A'..='Z' => {
                slug.push(character.to_ascii_lowercase());
                previous_was_dash = false;
            }
            ' ' | '-' | '_' => {
                if !previous_was_dash && !slug.is_empty() {
                    slug.push('-');
                    previous_was_dash = true;
                }
            }
            _ => {}
        }
    }

    slug.trim_matches('-').to_string()
}