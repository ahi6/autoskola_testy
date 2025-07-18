use dioxus::prelude::*;

use crate::utils::resolve_asset_path;
use downloader4etesty2::types::*;

#[component]
pub fn Answer(option: QuestionOption) -> Element {
    let mut is_toggled = use_signal(|| false);

    use_memo(use_reactive(&option, move |_option| {
        // Option changed
        // Turn off toggle state
        is_toggled.set(false);
    }));

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

    let color_class = if !is_toggled() {
        // Not toggled
        "bg-white"
    } else if option.is_correct {
        // Toggled & Correct
        "bg-green-400"
    } else {
        // Toggled & Wrong
        "bg-red-400"
    };

    let button_class =
        "border-2 border-gray-300 p-4 rounded-lg h-full w-full flex items-center justify-center "
            .to_string()
            + color_class;

    rsx! {
        button {
            class: "flex-1 block cursor-pointer",
            onclick: move |_e| {
                let prev = is_toggled();
                is_toggled.set(!prev);
            },
            div {
                class: button_class,
                {content},
            }
        }
        div {
            // Dummy div to tell tailwind what classes I want available
            class: "-z-100 absolute opacity-0 h-0 w-0 border-2 border-gray-300 bg-white p-4 rounded-lg h-full w-full flex items-center justify-center bg-white bg-green-400 bg-red-400"
        }
    }
}
