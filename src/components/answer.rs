use dioxus::prelude::*;

use crate::{utils::resolve_asset_path, Route};
use downloader4etesty2::types::*;

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
        Link {
            class: "flex-1 block cursor-pointer",
            to: Route::HomeView,
            div {
                class: "border-2 border-gray-300 bg-white p-4 rounded-lg h-full w-full flex items-center justify-center",
                {content}
            }
        }
    }
}
