#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::collections::HashSet;

use eframe::{egui, NativeOptions};
use egui::{
	CentralPanel, Frame, Ui, WidgetText,
};

use egui_dock::{DockArea, DockState, Style, SurfaceIndex, TabViewer};

fn main() -> eframe::Result<()> {
	std::env::set_var("RUST_BACKTRACE", "1");
	let options = NativeOptions {
		initial_window_size: Some(egui::vec2(1024.0, 1024.0)),
		..Default::default()
	};
	eframe::run_native(
		"My egui App",
		options,
		Box::new(|_cc| Box::<MyApp>::default()),
	)
}

struct MyContext {
	// level: u8,
	race: String,
	subrace: String,
	race_extras: Vec<String>,
	class: String,
	pub style: Option<Style>,
}

struct MyApp {
	context: MyContext,
	tree: DockState<String>,
}

impl TabViewer for MyContext {
	type Tab = String;

	fn title(&mut self, tab: &mut Self::Tab) -> WidgetText {
		tab.as_str().into()
	}

	fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {
		match tab.as_str() {
			"Race" => self.race(ui),
			"Class" => self.class(ui),
			_ => {
				ui.label(tab.as_str());
			}
		}
	}
}

impl MyContext {
	fn race(&mut self, ui: &mut Ui) {
		let races = vec!["Dwarf", "Elf", "Halfling", "Human", "Dragonborn", "Gnome", "Half-Elf", "Half-Orc", "Tiefling"];
		ui.heading("Race");
		ui.horizontal_wrapped(|ui| {
			races.iter().for_each(|race| {
				if ui.button(race.to_string()).clicked() {
					self.race = race.to_string().to_owned();
					self.subrace = "".to_string();
					self.race_extras = vec![];
				}
			});
		});
		let subraces = match self.race.as_str() {
			"Dwarf" => vec!["Hill Dwarf", "Mountain Dwarf"],
			"Elf" => vec!["High Elf", "Wood Elf", "Dark Elf"],
			"Halfling" => vec!["Lightfoot Halfling", "Stout Halfling"],
			"Human" => vec!["Human Regular", "Human Variant"],
			"Dragonborn" => vec![],
			"Gnome" => vec!["Forest Gnome", "Rock Gnome"],
			"Half-Elf" => vec![],
			"Half-Orc" => vec![],
			"Tiefling" => vec![],
			_ => vec![],
		};
		if !subraces.is_empty() {
			ui.separator();
			ui.heading("Subrace");
		}
		ui.horizontal_wrapped(|ui| {
			subraces.iter().for_each(|subrace| {
				if ui.button(subrace.to_string()).clicked() {
					self.subrace = subrace.to_string().to_owned();
				}
			});
		});
		let mut race_extras = match self.subrace.as_str() {
			"High Elf" => vec!["Cantrip", "Extra Language"],
			"Human Variant" => vec!["Ability Increase", "Skill", "Feat"],

			_ => vec![],
		};
		match self.race.as_str() {
			"Human" => {race_extras.push("Extra Language");},
			"Dragonborn" => {race_extras.push("Dragon Ancestry");},
			_ => {}
		}
		if !race_extras.is_empty() {
			race_extras.iter().for_each(|extra| {
				ui.separator();
				ui.heading(extra.to_string());
			});
		}
	}

	fn class(&mut self, ui: &mut Ui) {
		let classes = vec!["Barbarian", "Bard", "Cleric", "Druid", "Fighter", "Monk", "Paladin", "Ranger", "Rogue", "Sorcerer", "Warlock", "Wizard"];
		ui.heading("Class");
		ui.horizontal_wrapped(|ui| {
			classes.iter().for_each(|class| {
				if ui.button(class.to_string()).clicked() {
					self.class = class.to_string().to_owned();
				}
			})
		});
		ui.label(format!("CharData {} {} {:?} {}", self.race, self.subrace, self.race_extras, self.class));
	}
}

impl Default for MyApp {
	fn default() -> Self {
		let dock_state =
			DockState::new(vec!["Race".to_owned(), "Class".to_owned(), "Background".to_owned()]);
		let mut open_tabs = HashSet::new();

		for node in dock_state[SurfaceIndex::main()].iter() {
			if let Some(tabs) = node.tabs() {
				for tab in tabs {
					open_tabs.insert(tab.clone());
				}
			}
		}

		let context = MyContext {
			// level: 1,
			race: "Human".to_string(),
			subrace: "Human".to_string(),
			race_extras: vec![],
			class: "Fighter".to_string(),
			style: None,
		};

		Self {
			context,
			tree: dock_state,
		}
	}
}

impl eframe::App for MyApp {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		CentralPanel::default()
			// When displaying a DockArea in another UI, it looks better
			// to set inner margins to 0.
			.frame(Frame::central_panel(&ctx.style()).inner_margin(0.))
			.show(ctx, |ui| {
				let mut style = self
					.context
					.style
					.get_or_insert(Style::from_egui(ui.style()))
					.clone();
				style.tab_bar.height = 50.0;
				style.tab_bar.fill_tab_bar = true;

				DockArea::new(&mut self.tree)
					.style(style)
					.show_close_buttons(false)
					.draggable_tabs(false)
					.show_inside(ui, &mut self.context);
			});
	}
}