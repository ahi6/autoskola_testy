mod components;
mod utils;
mod views;

use dioxus::prelude::*;

use dioxus_sdk::set_dir;

use views::HomeView;
use views::QuestionView;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

pub const BASE_URL: &str = "http://127.0.0.1:8080";

fn main() {
    set_dir!();
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[route("/")]
    HomeView,

    #[route("/topic/:topic_title")]
    QuestionView { topic_title: String },
}
