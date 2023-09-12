// use leptos::*;
//
// #[allow(dead_code)]
// #[derive(Debug)]
// pub struct CharData {
//     class: String,
//     subclass: String,
// }
//
// impl Default for CharData {
//     fn default() -> Self {
//         Self {
//             class: "None".to_string(),
//             subclass: "None".to_string(),
//         }
//     }
// }
//
// #[derive(leptos::IntoView)]
// pub enum Class {
//     Barbarian,
//     Bard,
//     Cleric,
//     Druid,
//     Fighter,
//     Monk,
//     Paladin,
//     Ranger,
//     Rogue,
//     Sorcerer,
//     Warlock,
//     Wizard,
// }
// #[component]
// fn App(cx: Scope) -> impl IntoView {
//     let ( character, set_character ) = create_signal(cx, CharData::default());
//     let classes: Vec<Class> = vec![Class::Barbarian, Class::Bard, Class::Cleric, Class::Druid, Class::Fighter, Class::Monk, Class::Paladin, Class::Ranger, Class::Rogue, Class::Sorcerer, Class::Warlock, Class::Wizard];
//     let _selected_classes = classes.iter().map(|class| {
//         view! { cx, <div on:onclick=move |_| {
//             set_character.update( |character| *character = CharData { class: class, subclass: character.subclass.clone() } );
//         } > { class } </div> }
//     });
//     view! { cx,
//         <p>
//             {"Classes"}
//         </p>
//     }
// }
//
// fn main() {
//     leptos::mount_to_body(|cx| view! { cx, <App/> })
// }
use leptos::*;

pub struct CharData {
    name: String,
    class: String,
}

impl Default for CharData {
    fn default() -> Self {
        Self {
            name: "None".to_string(),
            class: "None".to_string(),
        }
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"Name"</h2>
        <Name/>
        <h2>"Class"</h2>
        <Class/>
    }
}

#[component]
fn Name(cx: Scope) -> impl IntoView {
    let (name, set_name) = create_signal(cx, CharData::default().name);

    view! { cx,
        <input type="text"
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }
            prop:value=name
        />
        <p>"Name is: " {name}</p>
    }
}

#[component]
fn Class(cx: Scope) -> impl IntoView {
    // create a signal to hold the value
    let (class, set_class) = create_signal(cx, CharData::default().class);
    let classes = &["Barbarian", "Bard", "Cleric", "Druid", "Fighter", "Monk", "Paladin", "Ranger", "Rogue", "Sorcerer", "Warlock", "Wizard"];
    view! { cx,
        <For
            each=move || classes
            key=move |class| class.to_string()
            view=move |cx, Class| {
                view! { cx,
                    <div on:click=move |_| {
                        set_class(Class.to_string());
                    } > { Class.to_string() } </div>
                }
            }
        />
        <div>"Class is: " {class}</div>
    }
}

fn main() {
    leptos::mount_to_body(|cx| view! { cx, <App/> })
}