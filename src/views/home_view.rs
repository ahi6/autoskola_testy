use dioxus::prelude::*;
use downloader4etesty2::types::*;

use crate::Route;

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
                            Link {
                                class: "text-2xl cursor-pointer",
                                to: Route::QuestionView {
                                    topic_title: {topic.title.clone()},
                                },
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
