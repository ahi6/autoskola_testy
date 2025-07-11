use dioxus::prelude::*;

use downloader4etesty2::types::*;

use crate::components::answer::Answer;
use crate::utils::resolve_asset_path;

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
