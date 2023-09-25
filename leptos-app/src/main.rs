use std::string::ToString;
use leptos::*;
use leptos::server_fn::serde::de::Unexpected::Char;
use once_cell::sync::Lazy;

struct CharData {
    name: String,
    race: String,
    subrace: String,
    class: String,
    subclass: String,
}

impl CharData {
    fn new() -> CharData {
        CharData {
            name: "None".to_string(),
            class: "None".to_string(),
            subclass: "None".to_string(),
            race: "None".to_string(),
            subrace: "None".to_string(),
        }
    }
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <h2>"Form"</h2>
        <Form/>
    }
}

#[component]
fn Form(cx: Scope) -> impl IntoView {
    let mut chardata = CharData::new();
    let (name, set_name) = create_signal(cx, chardata.name);
    let (race, set_race) = create_signal(cx, chardata.race);
    let races = vec!["Dwarf", "Elf", "Halfling", "Human", "Dragonborn", "Gnome", "Half-Elf", "Half-Orc", "Tiefling"];
    let (subrace, set_subrace) = create_signal(cx, chardata.subrace);
    let mut subraces: Vec<&str> = vec![];
    match race.get().as_str() {
       "Dwarf" => subraces = vec!["Hill Dwarf", "Mountain Dwarf"],
       "Elf" => subraces = vec!["High Elf", "Wood Elf", "Dark Elf"],
       "Halfling" => subraces = vec!["Lightfoot Halfling", "Stout Halfling"],
       "Human" => subraces = vec!["Human"],
       "Dragonborn" => subraces = vec!["Black", "Blue", "Brass", "Bronze", "Copper", "Gold", "Green", "Red", "Silver", "White"],
       "Gnome" => subraces = vec!["Forest Gnome", "Rock Gnome"],
       "Half-Elf" => subraces = vec!["Half-Elf"],
       "Half-Orc" => subraces = vec!["Half-Orc"],
       "Tiefling" => subraces = vec!["Tiefling"],
        &_ => unreachable!()
    };
    let (class, set_class) = create_signal(cx, chardata.class);
    let classes = vec!["Barbarian", "Bard", "Cleric", "Druid", "Fighter", "Monk", "Paladin", "Ranger", "Rogue", "Sorcerer", "Warlock", "Wizard"];
    view! { cx,
        <input type="text"
        on:input=move |ev| {
            set_name(event_target_value(&ev));
        }
        prop:value=name
        />
        <p>"Name is: " {name}</p>
        <For
            each=move || races.clone()
            key=move |race| race.to_string()
            view=move |cx, Race| {
                view! { cx,
                    <div on:click=move |_| {
                        set_race(Race.to_string());
                    } > { Race.to_string() } </div>
                }
            }
        />
        <div /*on:change=move |_| {
            view! {cx,
                <For
                    each=move || subraces.clone()
                    key=move |subrace| subrace.to_string()
                    view=move |cx, Subrace| {
                        view! { cx,
                            <div on:click=move |_| {
                                set_subrace(Subrace.to_string());
                            } > { Subrace.to_string() } </div>
                        }
                    }
                />
                <div>"Subrace is: " {subrace}</div>
            }
        }*/ >"Race is: " {race}</div>
        <For
            each=move || classes.clone()
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
    leptos::mount_to_body(|cx| view! { cx, <App/> });
}