use dioxus::prelude::*;

mod components;
mod repositories;
mod services;
mod stores;
mod types;
mod views;

use views::{CharacterProfile, Favorites, Home, Overall, Settings};

const MAIN_CSS: Asset = asset!("/assets/main.css");
const GOOGLE_FONTS_CSS: &str = "https://fonts.googleapis.com/css2?family=Cinzel:wght@400;500&family=Inter:wght@400;500&family=JetBrains+Mono:wght@400;500&display=swap";
const TABLER_ICONS_CSS: &str = "https://cdn.jsdelivr.net/npm/@tabler/icons-webfont@latest/dist/tabler-icons.min.css";

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub(crate) enum Route {
    #[route("/")]
    Home {},
    #[route("/overall")]
    Overall {},
    #[route("/character/:region/:realm/:name")]
    CharacterProfile { region: String, realm: String, name: String },
    #[route("/favorites")]
    Favorites {},
    #[route("/settings")]
    Settings {},
}

fn main() {
    let window = dioxus::desktop::WindowBuilder::new()
        .with_title("WoW Companion")
        .with_inner_size(dioxus::desktop::LogicalSize::new(1520.0, 940.0))
        .with_min_inner_size(dioxus::desktop::LogicalSize::new(1320.0, 860.0));

    LaunchBuilder::desktop()
        .with_cfg(
            dioxus::desktop::Config::new()
                .with_menu(None)
                .with_disable_context_menu(true)
                .with_window(window),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: GOOGLE_FONTS_CSS }
        document::Link { rel: "stylesheet", href: TABLER_ICONS_CSS }
        Router::<Route> {}
    }
}
