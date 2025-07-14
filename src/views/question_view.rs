use dioxus::prelude::*;
use downloader4etesty2::types::*;

use crate::components::TestQuestion;
use crate::BASE_URL;

#[component]
pub fn QuestionView(topic_title: String) -> Element {
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
        div {
            class: "flex flex-col justify-center h-screen bg-gray-100 p-8",
            {question_rsx}
        }

    }
}
