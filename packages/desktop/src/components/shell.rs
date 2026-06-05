use dioxus::prelude::*;

use crate::{
    services::{
        build_search_dropdown, load_search_history, parse_search, push_search_history,
        search_message, CompanionService, SearchHistoryEntry, SearchMessage,
        SearchMessageKind,
    },
    types::NavSection,
    Route,
};

#[component]
pub(crate) fn AppShell(
    current: NavSection,
    profile_link: Route,
    accent: &'static str,
    accent_muted: &'static str,
    children: Element,
) -> Element {
    let mut search_focus_token = use_signal(|| 0_u64);

    rsx! {
        div {
            class: "app-shell",
            style: "--color-accent: {accent}; --color-accent-muted: {accent_muted};",
            onkeydown: move |event| {
                let key = event.key().to_string();
                if event.modifiers().ctrl() && key.eq_ignore_ascii_case("k") {
                    event.prevent_default();
                    search_focus_token.set(search_focus_token() + 1);
                }
            },
            TopBar {
                current,
                profile_link: profile_link.clone(),
                search_focus_token,
            }
            main { class: "page-shell", {children} }
        }
    }
}

#[component]
fn TopBar(current: NavSection, profile_link: Route, search_focus_token: Signal<u64>) -> Element {
    let navigator = use_navigator();
    let favorite_profiles = CompanionService::mock().favorite_profiles();
    let mut search_query = use_signal(String::new);
    let mut recent_history = use_signal(load_search_history);
    let mut search_feedback = use_signal(|| Option::<SearchMessage>::None);
    let mut dropdown_open = use_signal(|| false);
    let mut selected_region = use_signal({
        let profile_link = profile_link.clone();
        move || route_region(&profile_link)
    });
    let inline_message = search_feedback()
        .or_else(|| search_message(&search_query()));
    let invalid_search = inline_message
        .as_ref()
        .map(|message| matches!(message.kind, SearchMessageKind::Error))
        .unwrap_or(false);
    let dropdown = build_search_dropdown(
        &search_query(),
        &selected_region(),
        &recent_history(),
        favorite_profiles.as_slice(),
    );

    use_effect(move || {
        let focus_token = search_focus_token();
        if focus_token > 0 {
            dropdown_open.set(true);
            search_feedback.set(None);
        }
    });

    rsx! {
        header { class: "topbar",
            div { class: "brand-lockup",
                Link { class: "brand-anchor", to: Route::Home {},
                    span { class: "brand-rune", "W" }
                    div { class: "brand-copy",
                        span { class: "eyebrow", "Desktop companion" }
                        strong { "WoW Companion" }
                    }
                }
            }
            nav { class: "top-nav",
                Link {
                    class: "{nav_class(current, NavSection::Home)}",
                    to: Route::Home {},
                    i { class: "ti ti-home-2" }
                    span { "Overview" }
                }
                Link {
                    class: "{nav_class(current, NavSection::Overall)}",
                    to: Route::Overall {},
                    i { class: "ti ti-layout-grid" }
                    span { "Overall" }
                }
                Link {
                    class: "{nav_class(current, NavSection::Character)}",
                    to: profile_link.clone(),
                    i { class: "ti ti-user" }
                    span { "Character" }
                }
                Link {
                    class: "{nav_class(current, NavSection::Favorites)}",
                    to: Route::Favorites {},
                    i { class: "ti ti-bookmark" }
                    span { "Favorites" }
                }
                Link {
                    class: "{nav_class(current, NavSection::Settings)}",
                    to: Route::Settings {},
                    i { class: "ti ti-settings" }
                    span { "Settings" }
                }
            }
            div { class: "top-tools",
                div { class: "search-wrap",
                    form {
                        class: if invalid_search { "search-shell invalid" } else { "search-shell" },
                        onsubmit: move |event| {
                            event.prevent_default();

                            if let Some(entry) = parse_search(&search_query(), &selected_region()) {
                                recent_history.set(push_search_history(entry.clone()));
                                search_feedback.set(None);
                                dropdown_open.set(false);
                                navigator.push(entry.to_route());
                            } else {
                                search_feedback
                                    .set(
                                        search_message(&search_query())
                                            .or_else(|| {
                                                Some(SearchMessage::error("Enter CharacterName-Realm"))
                                            }),
                                    );
                                dropdown_open.set(true);
                            }
                        },
                        i { class: "ti ti-search" }
                        input {
                            key: "topbar-search-{search_focus_token()}",
                            id: "topbar-search",
                            class: "search-input",
                            r#type: "text",
                            value: search_query(),
                            placeholder: "CharacterName-Realm",
                            autocomplete: "off",
                            spellcheck: "false",
                            autofocus: search_focus_token() > 0,
                            onfocus: move |_| {
                                dropdown_open.set(true);
                            },
                            onkeydown: move |event| {
                                if event.key().to_string() == "Escape" {
                                    dropdown_open.set(false);
                                }
                            },
                            oninput: move |event| {
                                search_query.set(event.value());
                                search_feedback.set(None);
                                dropdown_open.set(true);
                            },
                        }
                        button {
                            class: "search-submit",
                            r#type: "submit",
                            title: "Search character",
                            i { class: "ti ti-arrow-right" }
                        }
                    }
                    if let Some(message) = inline_message {
                        div { class: if matches!(message.kind, SearchMessageKind::Error) { "search-message error" } else { "search-message hint" },
                            "{message.text}"
                        }
                    }
                    if dropdown_open() {
                        div { class: "search-dropdown",
                            if !dropdown.recent.is_empty() {
                                div { class: "dropdown-section-label", "Recent" }
                                {
                                    dropdown
                                        .recent
                                        .iter()
                                        .map(|entry| {
                                            let entry = entry.clone();
                                            let navigator = navigator.clone();
                                            rsx! {
                                                button {
                                                    key: "recent-{entry.region}-{entry.realm_slug}-{entry.name}",
                                                    class: "dropdown-row",
                                                    r#type: "button",
                                                    onclick: move |_| {
                                                        recent_history.set(push_search_history(entry.clone()));
                                                        search_feedback.set(None);
                                                        dropdown_open.set(false);
                                                        navigator.push(entry.to_route());
                                                    },
                                                    i { class: "ti ti-history dropdown-row-icon" }
                                                    div { class: "dropdown-row-body",
                                                        span { class: "dropdown-name", "{entry.name}" }
                                                        span { class: "dropdown-row-meta",
                                                            span { class: "realm-name", "{entry.realm_name}" }
                                                            span { class: "region-tag", "{entry.region.to_ascii_uppercase()}" }
                                                        }
                                                    }
                                                }
                                            }
                                        })
                                }
                            }
                            if !dropdown.favorites.is_empty() {
                                if !dropdown.recent.is_empty() {
                                    div { class: "dropdown-divider" }
                                }
                                div { class: "dropdown-section-label", "Favorites" }
                                {
                                    dropdown
                                        .favorites
                                        .iter()
                                        .map(|entry| {
                                            let entry = entry.clone();
                                            let navigator = navigator.clone();
                                            rsx! {
                                                button {
                                                    key: "favorite-{entry.region}-{entry.realm_slug}-{entry.name}",
                                                    class: "dropdown-row",
                                                    r#type: "button",
                                                    onclick: move |_| {
                                                        recent_history.set(push_search_history(entry.clone()));
                                                        search_feedback.set(None);
                                                        dropdown_open.set(false);
                                                        navigator.push(entry.to_route());
                                                    },
                                                    i { class: "ti ti-heart dropdown-row-icon" }
                                                    div { class: "dropdown-row-body",
                                                        span { class: "dropdown-name", "{entry.name}" }
                                                        span { class: "dropdown-row-meta",
                                                            span { class: "realm-name", "{entry.realm_name}" }
                                                            span { class: "region-tag", "{entry.region.to_ascii_uppercase()}" }
                                                        }
                                                    }
                                                }
                                            }
                                        })
                                }
                            }
                            if !dropdown.realm_hints.is_empty() {
                                if !dropdown.recent.is_empty() || !dropdown.favorites.is_empty() {
                                    div { class: "dropdown-divider" }
                                }
                                div { class: "dropdown-section-label", "Realm Suggestions" }
                                {
                                    dropdown
                                        .realm_hints
                                        .iter()
                                        .map(|hint| {
                                            let hint = hint.clone();
                                            let navigator = navigator.clone();
                                            rsx! {
                                                button {
                                                    key: "realm-{hint.region}-{hint.realm_slug}",
                                                    class: "dropdown-row",
                                                    r#type: "button",
                                                    onclick: move |_| {
                                                        let name = search_query()
                                                            .split_once('-')
                                                            .map(|(name, _)| name.trim().to_string())
                                                            .unwrap_or_default();
                                                        let entry = SearchHistoryEntry {
                                                            name,
                                                            realm_name: hint.realm_name.clone(),
                                                            realm_slug: hint.realm_slug.clone(),
                                                            region: selected_region(),
                                                        };

                                                        recent_history.set(push_search_history(entry.clone()));
                                                        search_query.set(format!("{}-{}", entry.name, entry.realm_name));
                                                        search_feedback.set(None);
                                                        dropdown_open.set(false);
                                                        navigator.push(entry.to_route());
                                                    },
                                                    i { class: "ti ti-map-pin dropdown-row-icon" }
                                                    div { class: "dropdown-row-body",
                                                        span { class: "dropdown-name", "{hint.realm_name}" }
                                                        span { class: "dropdown-row-meta",
                                                            span { class: "realm-name", "Use selected region" }
                                                            span { class: "region-tag", "{hint.region.to_ascii_uppercase()}" }
                                                        }
                                                    }
                                                }
                                            }
                                        })
                                }
                            }
                            if dropdown.is_empty() {
                                div { class: "dropdown-empty",
                                    "Recent searches and realm hints appear here."
                                }
                            }
                        }
                    }
                }
                div { class: "region-switch",
                    button {
                        class: if selected_region() == "eu" { "region-pill active" } else { "region-pill" },
                        r#type: "button",
                        onclick: move |_| {
                            selected_region.set(String::from("eu"));
                            dropdown_open.set(true);
                        },
                        "EU"
                    }
                    button {
                        class: if selected_region() == "us" { "region-pill active" } else { "region-pill" },
                        r#type: "button",
                        onclick: move |_| {
                            selected_region.set(String::from("us"));
                            dropdown_open.set(true);
                        },
                        "US"
                    }
                }
                Link { class: "primary-link top-cta", to: profile_link, "Open Featured Profile" }
            }
        }
    }
}

fn nav_class(current: NavSection, section: NavSection) -> &'static str {
    if current == section { "nav-link active" } else { "nav-link" }
}

fn route_region(route: &Route) -> String {
    match route {
        Route::CharacterProfile { region, .. } => normalize_region(region).unwrap_or_else(|| String::from("eu")),
        _ => String::from("eu"),
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