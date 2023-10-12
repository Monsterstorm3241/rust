#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::any::Any;
use std::collections::HashSet;

use eframe::{egui, NativeOptions};
use egui::{
	CentralPanel, Frame, Ui, WidgetText,
};

use egui_dock::{DockArea, DockState, Style, SurfaceIndex, TabViewer};
// use catppuccin_egui::MOCHA;
use serde_json::{json, Value};

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

struct Characters {
	character_count: usize,
	character_value: Vec<Box<dyn ComponentVec>>,
}
impl Characters {
	fn new() -> Self {
		Self {
			character_count: 0,
			character_value: Vec::new(),
		}
	}
	fn new_character(&mut self) {
		let character_id = self.character_count;
		for ComponentVec in self.character_value.iter_mut() {
			ComponentVec.push_none();
		}
		self.character_count += 1;
		character_id;
	}
	fn add_value_to_character<ComponentType: 'static>(
		&mut self,
		id: usize,
		value: ComponentType,
	) {
		for component_vec in self.character_value.iter_mut() {
			if let Some(component_vec) = component_vec
				.as_any_mut()
				.downcast_mut::<Vec<Option<ComponentType>>>()
			{
				component_vec[id] = Some(value);
				return;
			}
		}

		// No matching component storage exists yet, so we have to make one.
		let mut new_character_value: Vec<Option<ComponentType>> =
			Vec::with_capacity(self.character_count);

		// All existing entities don't have this component, so we give them `None`
		for _ in 0..self.character_count {
			new_character_value.push(None);
		}

		// Give this Entity the Component.
		new_character_value[id] = Some(value);
		self.character_value.push(Box::new(new_character_value));
	}
}
trait ComponentVec {
	fn as_any(&self) -> &dyn std::any::Any;
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
	fn push_none(&mut self);
}
impl<T: 'static> ComponentVec for Vec<Option<T>> {
	fn as_any(&self) -> &dyn std::any::Any {
		self as &dyn std::any::Any
	}
	fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
		self as &mut dyn std::any::Any
	}
	fn push_none(&mut self) {
		self.push(None)
	}
}
struct Health(i32);
struct Name(&'static str);
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
			"Confirm" => self.confirm(ui),
			_ => {
				ui.label(tab.as_str());
			}
		}
	}
}

impl MyContext {
	fn race(&mut self, ui: &mut Ui) {
		let races = vec!["Dwarf", "Elf", "Halfling", "Human", "Dragonborn", "Gnome", "Half-Elf", "Half-Orc", "Tiefling"];
		let mut RACE: String = Default::default();
		ui.heading("Race");
		ui.horizontal_wrapped(|ui| {
			races.iter().for_each(|race| {
				if ui.button(race.to_string()).clicked() {
					RACE = race.to_string().to_owned();
					self.subrace = "".to_string();
					self.race_extras = vec![];
				}
			});
		});
		let subraces = match RACE.as_str() {
			"Dwarf" => vec!["Hill Dwarf", "Mountain Dwarf"],
			"Elf" => vec!["High Elf", "Wood Elf", "Dark Elf (Drow)"],
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
			ui.horizontal_wrapped(|ui| {
				subraces.iter().for_each(|subrace| {
					if ui.button(subrace.to_string()).clicked() {
						self.subrace = subrace.to_string().to_owned();
					}
				});
			});
		} else {
			self.subrace = RACE.clone();
		}
		let ability_increase = vec!["Ability Increase", "Strength", "Dexterity", "Constitution", "Intelligence", "Wisdom", "Charisma"];
		let cantrip = vec!["Cantrip", "Acid Splash", "Blade Ward", "Chill Touch", "Dancing Lights", "Fire Bolt", "Friends", "Light", "Mage Hand", "Mending", "Message", "Minor Illusion", "Poison Spray", "Prestidigitation", "Ray of Frost", "Shocking Grasp", "True Strike", "Vicious Mockery"];
		let dragon_ancestry = vec!["Dragon Ancestry", "Black", "Blue", "Brass", "Bronze", "Copper", "Gold", "Green", "Red", "Silver", "White",];
		let extra_language = vec!["Extra Language", "Dwarvish", "Elvish", "Giant", "Gnomish", "Goblin", "Halfling", "Orc", "Abyssal", "Celestial", "Draconic", "Deep Speech", "Infernal", "Primordial", "Sylvan", "Undercommon"];
		let feat = vec!["Feat", "Actor", "Alert", "Athlete", "Charger", "Crossbow Expert", "Defensive Duelist", "Dual Wielder", "Dungeon Delver", "Durable", "Elemental Adept", "Grappler", "Great Weapon Master", "Healer", "Heavily Armored", "Heavy Armor Master", "Inspiring Leader", "Keen Mind", "Lightly Armored", "Linguist", "Lucky", "Mage Slayer", "Magic Initiate", "Martial Adept", "Medium Armor Master", "Mobile", "Moderately Armored", "Mounted Combatant", "Observant", "Polearm Master", "Resilient", "Ritual Caster", "Savage Attacker", "Sentinel", "Sharpshooter", "Shield Master", "Skilled", "Skulker", "Spell Sniper", "Tavern Brawler", "Tough", "War Caster", "Weapon Master"];
		let skill = vec!["Skill", "Acrobatics", "Animal Handling", "Arcana", "Athletics", "Deception", "History", "Insight", "Intimidation", "Investigation", "Medicine", "Nature", "Perception", "Performance", "Persuasion", "Religion", "Sleight of Hand", "Stealth", "Survival"];
		let race_extras = match self.subrace.as_str() {
			"High Elf" => vec![cantrip, extra_language],
			"Human Regular" => vec![extra_language],
			"Human Variant" => vec![ability_increase, skill, feat, extra_language],
			"Dragonborn" => vec![dragon_ancestry],
			_ => vec![],
		};
		if !race_extras.is_empty() {
			race_extras.iter().for_each(|extra| {
				ui.separator();
				ui.heading(extra[0].to_string());
				ui.horizontal_wrapped(|ui| {
					extra[1..].iter().for_each(|extra| {
						// let mut button = ui.button(extra.to_string());
						// if button.clicked() {
						// 	self.race_extras.push(extra.to_string());
						// }

						if ui.button(extra.to_string()).clicked() {
							self.race_extras.push(extra.to_string());
						}
					});
				});
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
	}

	fn confirm(&mut self, ui: &mut Ui) {
		ui.heading("confirm your choices");
		ui.heading(format!("Race: {}", self.subrace));
		ui.heading(format!("Race extra's: {:?}", self.race_extras));
		ui.heading(format!("Class: {}", self.class));
		ui.label(format!("Style: {}", self.style.as_ref().unwrap().tab_bar.height));
	}
}

impl Default for MyApp {
	fn default() -> Self {
		let dock_state =
			DockState::new(vec!["Race".to_owned(), "Class".to_owned(), "Background".to_owned(), "Confirm".to_owned()]);
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
			subrace: "Human Regular".to_string(),
			race_extras: vec![],
			class: "Barbarian".to_string(),
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
		// catppuccin_egui::set_theme(ctx, MOCHA);
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