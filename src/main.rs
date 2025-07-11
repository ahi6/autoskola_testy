mod components;
mod utils;

use dioxus::prelude::*;

use downloader4etesty2::types::*;

use components::TestQuestion;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

pub const BASE_URL: &str = "http://127.0.0.1:8080";

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let placeholder_question = Question::default();

    static PATH_TO_QUESTIONS: Asset = asset!("/assets/output/Znalost dopravních znače.json");

    let questions_fut: Resource<Vec<Question>> = use_resource(move || async move {
        let questions_json = reqwest::get(format!("{}{}", BASE_URL, PATH_TO_QUESTIONS))
            .await
            .expect("Failed to fetch questions")
            .text()
            .await
            .expect("Failed to read response text");
        serde_json::from_str(&questions_json).expect("Failed to parse JSON")
    });

    let question_rsx = match &*questions_fut.read_unchecked() {
        Some(questions) => {
            let question = questions
                .get(0)
                .cloned()
                .unwrap_or(placeholder_question.clone());
            rsx! {
                TestQuestion { question }
            }
        }
        None => {
            rsx! {
                p {
                    "No questions available"
                }
            }
        }
    }
    .unwrap();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        {question_rsx}
    }
}
