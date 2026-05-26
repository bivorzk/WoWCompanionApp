use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/character/:region/:realm/:name")]
    CharacterProfile {
        region: String,
        realm: String,
        name: String,
    },
    #[route("/favorites")]
    Favorites {},
    #[route("/settings")]
    Settings {},
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div {
            h1 { "Home" }
            input { r#type: "search", placeholder: "Search character placeholder" }
        }
    }
}

#[component]
fn CharacterProfile(region: String, realm: String, name: String) -> Element {
    rsx! {
        div {
            h1 { "CharacterProfile" }
            p { "{region}/{realm}/{name}" }
        }
    }
}

#[component]
fn Favorites() -> Element {
    rsx! {
        div { "Favorites" }
    }
}

#[component]
fn Settings() -> Element {
    rsx! {
        div { "Settings" }
    }
}
