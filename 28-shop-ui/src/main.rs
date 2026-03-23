use dioxus::prelude::*;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut clicks = use_signal(|| 0_u32);
    rsx! {
        h1 { "Shop UI" }
        p { "Button presses: {clicks}" }
        button {
            onclick: move |_| *clicks.write() += 1,
            "Press me"
        }
    }
}
