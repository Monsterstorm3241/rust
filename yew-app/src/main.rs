use std::rc::Rc;
use yew::prelude::*;

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
    let character = Rc::new(CharData::default());
    let classes = vec!["Barbarian", "Bard", "Cleric", "Druid", "Fighter", "Monk", "Paladin", "Ranger", "Rogue", "Sorcerer", "Warlock", "Wizard"];
    let selected_classes = classes.iter().map(|class| {
        let character = Rc::clone(&character);
        let on_click = Callback::from(move |event: MouseEvent| {
            event.prevent_default();
            let class = class.to_owned();
            character.borrow_mut().change_class(class.to_string());
        });
        html! { <div onclick={on_click}> { class } </div> }
    }).collect::<Html>();
    
    html! {
        <div>
            <h1>{ "Classes" }</h1>
            <div>
                { selected_classes }
                { &character.borrow().class[..] }
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}