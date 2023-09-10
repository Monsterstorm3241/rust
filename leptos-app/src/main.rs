use leptos::*;

#[allow(dead_code)]
pub struct CharData {
    class: String,
    subclass: String,
}

impl Default for CharData {
    fn default() -> Self {
        Self {
            class: "None".to_string(),
            subclass: "None".to_string(),
        }
    }
}
#[component]
fn App(cx: Scope) -> impl IntoView {
    let mut character = CharData::default();
    view! { cx,
        <div
            on:click=move |_| {
                character.class = "clicked".to_string();
            }
        >
            "Click me"
        </div>
        <p>
            {"aaaaaaaaah"}
        </p>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
