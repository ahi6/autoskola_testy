mod components;
mod utils;

use dioxus::prelude::*;

use dioxus_sdk::set_dir;
use downloader4etesty2::types::*;

use components::TestQuestion;

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

#[component]
fn QuestionView() -> Element {
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

#[component]
pub fn HomeView() -> Element {
    // Persistent Version:
    let mut topics_signal: Signal<Option<Vec<Topic>>> = dioxus_sdk::storage::use_synced_storage::<
        dioxus_sdk::storage::LocalStorage,
        Option<Vec<Topic>>,
    >("topics".to_string(), || None);
    //
    // In-memory only version:
    // let mut topics_signal: Signal<Option<Vec<Topic>>> =
    //     dioxus_sdk::storage::use_persistent("topicss", || None);

    let topics_cloned = topics_signal.read().clone();

    // let topics_cloned = Some(
    //     [Topic {
    //         title: "a".to_string(),
    //         url: "b".to_string(),
    //     }]
    //     .to_vec(),
    // );

    match topics_cloned {
        Some(topics) => rsx! {
            div {
                class: "flex flex-col justify-center items-center h-screen bg-gray-100 p-8 gap-4",
                h1 {
                    class: "text-5xl font-bold md-4",
                    "Pick a topic"
                }
                ul {
                    for topic in topics {
                        li {
                            a {
                                class: "text-2xl",
                                href: "#",
                                "{topic.title}"
                            }
                        }
                    }


                    button {
                        class: "text-xl text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800",
                        onclick: move |_e| {
                            *topics_signal.write() = None;
                        },
                        "Clear topic memory"
                    }
                    // p {
                    //     {format!("{:?}", topics_signal.read())}
                    // }
                }
            }
        },
        None => rsx! {
            div {
                class: "flex flex-col justify-center items-center h-screen bg-gray-100 p-8 gap-8",
                h1 {
                    class: "text-5xl font-bold md-4",
                    "Welcome"
                }
                p {
                    class: "text-xl",
                    "Please begin by downloading some topics"
                }
                button {
                    class: "text-xl text-white bg-blue-700 hover:bg-blue-800 focus:ring-4 focus:ring-blue-300 font-medium rounded-lg px-5 py-2.5 me-2 mb-2 dark:bg-blue-600 dark:hover:bg-blue-700 focus:outline-none dark:focus:ring-blue-800",
                    onclick: move |_e| {
                        spawn(async move {
                            let topics = download_topics().await;
                            *topics_signal.write() = Some(topics);
                            }
                        );

                    },
                    "Download"
                }
            }
        },
    }
}

async fn download_topics() -> Vec<Topic> {
    let extractor = downloader4etesty2::extractor::Extractor::new();
    extractor.fetch_bulletin_topics().await.unwrap()
}

#[derive(Routable, Clone, PartialEq)]
enum Route {
    #[route("/")]
    QuestionView,

    #[route("/home")]
    HomeView,
}
