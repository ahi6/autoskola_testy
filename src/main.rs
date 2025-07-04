use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        TestQuestion {}

    }
}

#[component]
pub fn TestQuestion() -> Element {
    rsx! {
        div {
            class: "flex flex-col items-center h-screen bg-gray-100 p-8",
            div {
                id: "question",
                class: "w-full border-2 border-gray-300 bg-white p-4 rounded-lg p-8 m-8 flex flex-col items-center justify-center gap-4",
                p {
                    class: "text-xs font-extralight",
                    "Question ID" }
                p {
                    class: "text-4xl font-bold",
                    "Question text" }
                img { src: "https://upload.wikimedia.org/wikipedia/commons/thumb/1/1e/Vienna_Convention_road_sign_B2a.svg/1024px-Vienna_Convention_road_sign_B2a.svg.png", width: "100px", height: "100px", height: "100px" }
            }
            div {
                id: "answers",
                class: "w-full flex flex-row items-center justify-center gap-2",
                Answer {}
                Answer {}
                Answer {}
            }
        }
    }
}

#[component]
pub fn Answer() -> Element {
    rsx! {
        a {
            class: "block w-1/3",
            href: "#",
            div {
                class: "border-2 border-gray-300 bg-white p-4 rounded-lg",
                p { "Answer text" }
            }
        }
    }
}
