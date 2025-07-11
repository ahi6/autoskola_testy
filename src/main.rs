use dioxus::prelude::*;

use downloader4etesty2::types::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

const BASE_URL: &str = "http://127.0.0.1:8080";

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

#[component]
pub fn TestQuestion(question: Question) -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center h-screen bg-gray-100 p-8",
            div {
                id: "question",
                class: "w-full border-2 border-gray-300 bg-white p-4 rounded-lg p-8 m-8 flex flex-col items-center justify-center gap-4",
                p {
                    class: "text-xs font-extralight",
                    "{question.code}" }
                p {
                    class: "text-4xl font-bold",
                    "{question.question_text}" }
                if let Some(image) = question.question_image {
                    img {
                        src: resolve_asset_path(&image),
                        width: "100px",
                        height: "100px"
                    }
                }
            }
            div {
                id: "answers",
                class: "w-full flex flex-row justify-between items-stretch gap-2",
                Answer { option: question.option_a }
                Answer { option: question.option_b }
                if let Some(option_c) = question.option_c {
                    Answer { option: option_c }
                }
            }
        }
    }
}

#[component]
pub fn Answer(option: QuestionOption) -> Element {
    let content = match option.content {
        QuestionOptionType::Text(text) => rsx! {
            p {
                {text}
            }
        },
        QuestionOptionType::Image(image) => rsx! {
            img {
                src: "{resolve_asset_path(&image)}",
                width: "100px",
                height: "100px",
            }
        },
    };

    rsx! {
        a {
            class: "flex-1 block",
            href: "#",
            div {
                class: "border-2 border-gray-300 bg-white p-4 rounded-lg h-full w-full flex items-center justify-center",
                {content}
            }
        }
    }
}

fn resolve_asset_path(relative_url: &str) -> String {
    format!("{}/assets/output{}", BASE_URL, relative_url)
}
