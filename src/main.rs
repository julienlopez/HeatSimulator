use dioxus::prelude::*;

mod components;
mod logic;

use crate::{components::Screen, logic::Infrastructure};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/styling/main.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let infrastructure = use_signal(|| Infrastructure::default());

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        Screen {}
    }
}
