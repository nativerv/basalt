use eframe::egui;

use crate::features::configuration::Configuration;
use crate::features::note_graph::NoteGraphUi;
use crate::features::veins::{VeinId, VeinSelectionUi, Veins};
use directories::ProjectDirs;
use egui::{CentralPanel, Color32, Event, Key, RichText};
use std::fs::File;
use std::io;
use std::rc::Rc;

const CONFIG_NOT_EXISTS_ERROR_TEXT: &str = r#"
Please create a file ~/.config/basalt/basalt.json and add
{
"veins": ["/path/to/your/notes/folder"]
}
to it (must be an absolute path)
"#;

/// Global Basalt state
pub struct BasaltApp {
  basalt_dirs: ProjectDirs,

  //configuration_path: PathBuf,
  configuration: Configuration,

  /// A list of veins known to the this Basalt instance.
  veins: Veins,
  current_vein: Option<VeinId>,

  note_graph_ui: Option<NoteGraphUi>,
}

impl Default for BasaltApp {
  fn default() -> Self {
    // FIXME(portability): mobile

    // FIXME: maybe this should be global? Or fork the `directories` and add constructor into
    // custom dirs for `BasaltApp:;from_configuration` and tests?
    let basalt_dirs = ProjectDirs::from("com", "basalt", "basalt").unwrap_or_else(|| {
      const MESSAGE: &str =
        "could not retrieve valid home directory path for your OS: required for config dir";
      log::error!("{MESSAGE}");
      // FIXME(error presentation): panic
      panic!("{MESSAGE}")
    });

    let configuration_path = basalt_dirs.config_dir().join(Self::CONFIG_FILE_NAME);

    let configuration = File::open(configuration_path)
      // FIXME(error presentation): on invalid config, it will appear as though there is config with no veins.
      .and_then(|mut x| Configuration::read_configuration(&mut x))
      .unwrap_or_default();

    // Some veins can exist, some can error out...
    let veins = Veins::from_configuration(&configuration);

    let current_vein = None;
    let note_graph_ui = None;

    let mut basalt = Self {
      basalt_dirs,
      veins,
      current_vein,
      configuration,
      note_graph_ui,
    };

    // FIXME(error presentation): with this you should deduce error
    let _ = basalt.reload();

    basalt
  }
}

impl BasaltApp {
  const CONFIG_FILE_NAME: &str = "basalt.json";

  fn read_configuration(&mut self) -> io::Result<()> {
    let configuration_path = self.basalt_dirs.config_dir().join(Self::CONFIG_FILE_NAME);
    let configuration = File::open(configuration_path)
      // FIXME(error presentation): on invalid config, it will appear as though there is config with no veins.
      .and_then(|mut x| Configuration::read_configuration(&mut x))
      .unwrap_or_default();
    self.configuration = configuration;
    Ok(())
  }

  fn reload(&mut self) -> io::Result<()> {
    self.read_configuration()?;
    self.veins = Veins::from_configuration(&self.configuration);
    // FIXME: only one (the first) Vein is taken
    self.current_vein = self
      .veins
      .iter()
      .next()
      .map(|(vein_id, ..)| vein_id.clone());
    self.note_graph_ui = self
      .current_vein
      .as_ref()
      .and_then(|vein_id| self.veins.get_vein(vein_id))
      .and_then(|maybe_vein| maybe_vein.as_ref().map(Rc::clone).map(NoteGraphUi::new).ok());
    Ok(())
  }

  fn prev_vein(&mut self) {
    let prev_vein_id = self
      .veins
      .iter()
      .rev()
      .skip_while(|(&ref id, ..)| Some(id) != self.current_vein.as_ref())
      .map(|(id, ..)| id.clone())
      .nth(1);
    self.current_vein = prev_vein_id.or_else(|| self.current_vein.clone());
  }

  fn next_vein(&mut self) {
    let next_vein_id = self
      .veins
      .iter()
      .skip_while(|(&ref id, ..)| Some(id) != self.current_vein.as_ref())
      .map(|(id, ..)| id.clone())
      .nth(1);
    self.current_vein = next_vein_id.or_else(|| self.current_vein.clone());
  }

  fn handle_global_keys(&mut self, ctx: &egui::Context) {
    use egui::Modifiers;

    let mods = ctx.input(|input| input.modifiers);
    #[rustfmt::skip]
    match mods {
      Modifiers { alt: true, .. } => ctx.set_debug_on_hover(true),
      Modifiers { alt: false, .. } => ctx.set_debug_on_hover(false),
      _ => {},
    }

    // PERF: `Vec` clone each frame
    let events = ctx.input(|input| input.events.clone());
    for event in events.iter() {
      #[rustfmt::skip]
      match event {
        Event::Key { key: Key::K, pressed: true, .. } => self.prev_vein(),
        Event::Key { key: Key::J, pressed: true, .. } => self.next_vein(),
        Event::Key { key: Key::R, pressed: true, .. } => self.reload().expect("FIXME"),
        _ => {},
      }
    }
  }
}

impl eframe::App for BasaltApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    self.handle_global_keys(&ctx);

    CentralPanel::default().show(ctx, |ui| {
      ui.add(VeinSelectionUi::new(&self.veins, &mut self.current_vein));
      if let Some(ref mut note_graph_ui) = &mut self.note_graph_ui {
        note_graph_ui.ui(ui);
      } else {
        ui.label(RichText::new(CONFIG_NOT_EXISTS_ERROR_TEXT.trim()).color(Color32::RED));
      }
    });
  }
}
