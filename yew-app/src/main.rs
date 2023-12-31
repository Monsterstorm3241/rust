use std::rc::Rc;
use yew::prelude::*;
use web_sys::HtmlInputElement;

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

impl CharData {
    pub fn change_class(&mut self, class: String) {
        self.class = class;
    }
}

#[function_component]
fn App() -> Html {
    let input_node_ref = use_node_ref();
    let input_value_handle = use_state(String::default);
    let input_value = (*input_value_handle).clone();

    let character = Rc::new(CharData::default());
    let classes = vec!["Barbarian", "Bard", "Cleric", "Druid", "Fighter", "Monk", "Paladin", "Ranger", "Rogue", "Sorcerer", "Warlock", "Wizard"];
    let selected_classes = classes.iter().map(|class| { html! { <div> { class } </div> }}).collect::<Html>();
    
    html! {
        <div>
            <h1>{ "Classes" }</h1>
            <div>
                { selected_classes }
                { &character.class[..] }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}