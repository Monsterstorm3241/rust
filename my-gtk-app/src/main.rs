use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Grid, Paned};

const APP_ID: &str = "org.gtk_rs.HelloWorld2";

fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder().application_id(APP_ID).build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

fn build_ui(app: &Application) {
    let grid = Grid::new();
    let classes:Vec<String> =
        vec!["artificer", "barbarian", "bard", "cleric", "druid", "fighter", "monk", "paladin", "ranger", "rogue", "sorcerer", "warlock", "wizard"]
            .iter().map(|class| class.to_string()).collect();
    let class_container = Grid::new();
    class_container.set_column_homogeneous(true);
    let half = ((classes.len() / 2) as f32).ceil() as usize;
    for (pos, class) in classes.iter().enumerate() {
        let button = Button::builder()
            .label(class.clone())
            .build();
        button.connect_clicked(move |button| {
            println!("{}", button.label().unwrap());
            subclasses(button.label().unwrap().to_string());
        });
        class_container.attach(&button, { if pos <= half {pos} else {pos - half - 1}} as i32, if pos <= half { 0 } else { 1 }, 1, 1);
    };
    grid.attach(&class_container, 0, 0, 1, 1);
    fn subclasses(class: String) {
        let subclasses:Vec<String> = match class.as_str() {
            "artificer" => vec!["alchemist", "armorer", "artillerist", "battle smith"],
            "barbarian" => vec!["ancestral guardian", "battlerager", "berserker", "storm herald", "totem warrior", "zealot"],
            "bard" => vec!["creation", "eloquence", "glamour", "lore", "swords", "valor", "whispers"],
            "cleric" => vec!["arcana", "death", "forge", "grave", "knowledge", "life", "light", "nature", "order", "peace", "tempest", "trickery", "twilight", "war"],
            "druid" => vec!["dreams", "land", "moon", "shepherd", "spores", "stars", "wildfire"],
            "fighter" => vec!["arcane archer", "banneret", "battle master", "cavalier", "champion", "echo knight", "eldritch knight", "psi warrior", "rune knight", "samurai"],
            "monk" => vec!["astral self", "drunken master", "four elements", "kensei", "long death", "mercy", "open hand", "shadow", "sun soul", "tranquility"],
            "paladin" => vec!["ancients", "conquest", "crown", "devotion", "glory", "redemption", "vengeance", "watchers"],
            "ranger" => vec!["beast master", "fey wanderer", "gloom stalker", "horizon walker", "hunter", "monster slayer", "swarm keeper"],
            "rogue" => vec!["arcane trickster", "assassin", "inquisitive", "mastermind", "phantom", "scout", "soulknife", "swashbuckler", "thief"],
            "sorcerer" => vec!["aberrant mind", "clockwork soul", "divine soul", "draconic bloodline", "shadow magic", "storm sorcery", "wild magic"],
            "warlock" => vec!["archfey", "celestial", "fathomless", "fiend", "genie", "great old one", "hexblade", "undying"],
            "wizard" => vec!["abjuration", "bladesinging", "chronurgy", "conjuration", "divination", "enchantment", "evocation", "graviturgy", "illusion", "invention", "lore mastery", "necromancy", "onmancy", "psionics", "pyromancy", "transmutation", "war magic"],
            _ => vec![]
        }.iter().map(|subclass| subclass.to_string()).collect();
        let subclass_container = Grid::new();
        subclass_container.set_column_homogeneous(true);
        let half = ((subclasses.len() / 2) as f32).ceil() as usize;
        for (pos, subclass) in subclasses.iter().enumerate() {
            let button = Button::builder()
                .label(subclass.clone())
                .build();
            button.connect_clicked(move |button| {
                println!("{}", button.label().unwrap());
            });
            subclass_container.attach(&button, { if pos <= half {pos} else {pos - half - 1}} as i32, if pos <= half { 0 } else { 1 }, 1, 1);
        };
        // grid.attach(&subclass_container, 1, 0, 1, 1);
    }

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&grid)
        .build();

    // Present window
    window.present();
}