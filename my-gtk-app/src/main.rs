use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Grid};

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
    let classes:Vec<&'static str> = vec!["artificer", "barbarian", "bard", "cleric", "druid", "fighter", "monk", "paladin", "ranger", "rogue", "sorcerer", "warlock", "wizard"];
    let container = Grid::new();
    container.set_column_homogeneous(true);
    let half = ((classes.len() / 2) as f32).ceil() as usize;
    for (pos, class) in classes.iter().enumerate() {
        let button = Button::builder()
            .label(*class)
            .margin_top(12)
            .margin_bottom(12)
            .margin_start(12)
            .margin_end(12)
            .build();
        button.connect_clicked(move |_| {
            println!("{}", class);
        });
        container.attach(&button, { if pos <= half {pos} else {pos - half - 1}} as i32, if pos <= half { 0 } else { 1 }, 1, 1);
    };
    // Create a button with label and margins
    let button = Button::builder()
        .label("Press me!")
        .margin_top(12)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();
    container.attach(&button, 0, 2, 1, 1);

    // Connect to "clicked" signal of `button`
    button.connect_clicked(|button| {
        // Set the label to "Hello World!" after the button has been clicked on
        button.set_label("Hello World!");
    });

    // Create a window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("My GTK App")
        .child(&container)
        .build();

    // Present window
    window.present();
}