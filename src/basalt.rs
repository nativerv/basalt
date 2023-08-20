use eframe::egui;

use crate::features::configuration::Configuration;
use crate::features::note_graph::NoteGraphUi;
use crate::features::veins::Veins;
use directories::ProjectDirs;
use std::fs::File;

/// Global Basalt state
pub struct BasaltApp {
  //configuration_path: PathBuf,
  configuration: Configuration,

  // A list of veins known to the this Basalt instance. The head is the current vein.
  veins: Veins,

  note_graph_ui: NoteGraphUi,
}

impl BasaltApp {
  const CONFIG_FILE_NAME: &str = "basalt.json";

  fn from_configuration(configuration: Configuration) -> Self {
    let veins = Veins::from_configuration(&configuration);
    // FIXME: only one (first) Vein is taken it panics if there is none
    let note_graph_ui = NoteGraphUi::new(
      veins
        .iter()
        .next()
        .map(|(_, vein)| std::rc::Rc::clone(vein))
        .unwrap_or_else(|| panic!("reeeeeeeee!")),
    );
    Self {
      veins,
      configuration,
      note_graph_ui,
    }
  }
}

impl Default for BasaltApp {
  fn default() -> Self {
    // WARNING(portability): mobile

    // FIXME: maybe this should be global? Or fork the `directories` and add constructor into
    // custom dirs for `BasaltApp:;from_configuration` and tests?
    let basalt_dirs = ProjectDirs::from("com", "basalt", "basalt").unwrap_or_else(|| {
      const MESSAGE: &str =
        "could not retrieve valid home directory path for your OS: required for config dir";
      log::error!("{MESSAGE}");
      // FIXME: panic
      panic!("{MESSAGE}")
    });

    let configuration_path = basalt_dirs.config_dir().join(BasaltApp::CONFIG_FILE_NAME);

    let configuration = File::options()
      .read(true)
      .open(configuration_path)
      .into_iter()
      .flat_map(|mut x| Configuration::read_configuration(&mut x))
      .next()
      // FIXME: error presentation
      .unwrap_or_default();

    Self::from_configuration(configuration)
  }
}

impl eframe::App for BasaltApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      self.note_graph_ui.ui(ui);
    });
  }
}
