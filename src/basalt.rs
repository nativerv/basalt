use eframe::egui;

use crate::features::configuration::Configuration;
use crate::features::note_graph::NoteGraphUi;
use crate::features::veins::{Vein, VeinId, Veins};
use directories::ProjectDirs;
use std::collections::HashMap;
use std::fs::File;

/// Global Basalt state
pub struct BasaltApp {
  basalt_dirs: ProjectDirs,
  //configuration_path: PathBuf,
  configuration: Configuration,

  // A list of veins known to the this Basalt instance. The head is the current vein.
  veins: Veins,

  note_graph_ui: NoteGraphUi,
}

impl Default for BasaltApp {
  fn default() -> Self {
    // WARNING(portability): mobile

    let basalt_dirs = ProjectDirs::from("com", "basalt", "basalt").unwrap_or_else(|| {
      const MESSAGE: &str =
        "could not retrieve valid home directory path for your OS: required for config dir";
      log::error!("{MESSAGE}");
      // FIXME: panic
      panic!("{MESSAGE}")
    });

    let configuration_path = basalt_dirs.config_dir().join("basalt.json");

    let configuration = File::options()
      .read(true)
      .open(configuration_path)
      .into_iter()
      .flat_map(|mut x| Configuration::read_configuration(&mut x))
      .next()
      // FIXME: error presentation
      .unwrap_or_default();

    Self {
      basalt_dirs,
      veins: Veins::load_from_config(&configuration),
      configuration,
      note_graph_ui: Default::default(),
      // Doesn't compile if put here instead of above LUL
      // rust moment
      //veins: Veins::load_from_config(&configuration),
    }
  }
}

impl eframe::App for BasaltApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      self.note_graph_ui.ui(ui);
    });
  }
}
