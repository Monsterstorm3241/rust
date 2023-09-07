use leptos::*;

// The #[component] macro marks a function as a reusable component
// Components are the building blocks of your user interface
// They define a reusable unit of behavior
#[component]
fn App(cx: Scope) -> impl IntoView {
    // here we create a reactive signal
    // and get a (getter, setter) pair
    // signals are the basic unit of change in the framework
    // we'll talk more about them later
    let (text, set_text) = create_signal(cx, "Hello, world!");

    // the `view` macro is how we define the user interface
    // it uses an HTML-like format that can accept certain Rust values
    view! { cx,
        <button
            // on:click will run whenever the `click` event fires
            // every event handler is defined as `on:{eventname}`

            // we're able to move `set_count` into the closure
            // because signals are Copy and 'static
            on:click=move |_| {
                set_text.update(|n| *n = "You clicked me!");
            }
        >
            // text nodes in RSX should be wrapped in quotes,
            // like a normal Rust string
            "Click me"
        </button>
        <p>
            // you can insert Rust expressions as values in the DOM
            // by wrapping them in curly braces
            // if you pass in a function, it will reactively update
            {text}
        </p>
    }
}

// This `main` function is the entry point into the app
// It just mounts our component to the <body>
// Because we defined it as `fn App`, we can now use it in a
// template as <App/>
fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}
