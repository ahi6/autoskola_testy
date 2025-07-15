use std::hash::{DefaultHasher, Hash, Hasher};

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

    let mut questions_signal: Signal<Option<Result<Vec<Question>, String>>> =
        dioxus_sdk::storage::use_synced_storage::<
            dioxus_sdk::storage::LocalStorage,
            Option<Result<Vec<Question>, String>>,
        >(calculate_hash(&topic_title).to_string(), || None);

    // let question_rsx = match &*questions_fut.read_unchecked() {
    //     Some(questions) => {
    //         let question = questions
    //             .get(0)
    //             .cloned()
    //             .unwrap_or(placeholder_question.clone());
    //         rsx! {
    //             TestQuestion { question }
    //         }
    //     }
    //     None => {
    //         rsx! {
    //             p {
    //                 "No questions available"
    //             }
    //         }
    //     }
    // }
    // .unwrap();
    //
    let questions_cloned = questions_signal.read().clone();
    let question_rsx = match questions_cloned {
        Some(Ok(questions)) => rsx! {
            p {
                "Questions loaded"
            }
            p {
                {format!("{:?}", questions)}
            }
        },
        Some(Err(error_msg)) => rsx! {
            p {
                "An error occurred while loading questions:"
            }
            p {
                {error_msg}
            }
        },
        None => rsx! {
            p {
                onclick: move |_e| {
                    let topic_result = lookup_topic_by_title(&topic_title);
                    match topic_result {
                        Ok(topic) => {
                            spawn(async move {
                                let questions = download_questions(&topic).await;
                                *questions_signal.write() = Some(questions);
                                }
                            );
                        },
                        Err(err_msg) => {
                            *questions_signal.write() = Some(Err(err_msg));
                        },
                    }
                },
                "Click here to load questions"
            }
        },
    };

    rsx! {
        div {
            class: "flex flex-col justify-center h-screen bg-gray-100 p-8",
            {question_rsx}
        }

    }
}

fn lookup_topic_by_title(topic_title: &str) -> Result<Topic, String> {
    let topics_signal: Signal<Option<Vec<Topic>>> = dioxus_sdk::storage::use_synced_storage::<
        dioxus_sdk::storage::LocalStorage,
        Option<Vec<Topic>>,
    >("topics".to_string(), || None);
    let topics = topics_signal.read();

    let error_msg = format!(
        "Couldn't find question set with the title: \"{}\"",
        topic_title
    );

    match &*topics {
        Some(topics_vec) => {
            let topic = topics_vec
                .iter()
                .filter(|el| el.title == topic_title)
                .next();
            if let Some(borrowed_topic) = topic {
                return Ok(borrowed_topic.clone());
            } else {
                return Err(error_msg);
            }
        }
        None => return Err(error_msg),
    }
}

// https://doc.rust-lang.org/std/hash/index.html
// Used for unique and not overly long filenames
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

// Returns String instead of en error because it has to be cloneable
async fn download_questions(topic: &Topic) -> Result<Vec<Question>, String> {
    let extractor = downloader4etesty2::extractor::Extractor::new();
    extractor
        .fetch_questions(&topic.url)
        .await
        .map_err(|err| err.to_string())
}
