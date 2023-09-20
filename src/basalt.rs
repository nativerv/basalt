use eframe::egui;

use crate::features::configuration::Configuration;
use crate::features::note_graph::NoteGraphUi;
use crate::features::veins::Veins;
use directories::ProjectDirs;
use std::fs::File;
use std::rc::Rc;
use std::io;

/// Global Basalt state
pub struct BasaltApp {
  basalt_dirs: ProjectDirs,

  //configuration_path: PathBuf,
  configuration: Configuration,

  /// A list of veins known to the this Basalt instance.
  veins: Veins,

  note_graph_ui: Option<NoteGraphUi>,
}

impl BasaltApp {
  const CONFIG_FILE_NAME: &str = "basalt.json";

  fn from_configuration(configuration: Configuration) -> Self {
    panic!()
  }

  fn read_configuration(&mut self) -> io::Result<()> {
    let configuration_path = self.basalt_dirs.config_dir().join(Self::CONFIG_FILE_NAME);
    let configuration = File::open(configuration_path)
      // FIXME(error presentation): on invalid config, it will appear as though there is config with no veins.
      .and_then(|mut x| Configuration::read_configuration(&mut x))
      .unwrap_or_default();
    self.configuration = configuration;

    Ok(())
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

    let configuration_path = basalt_dirs.config_dir().join(Self::CONFIG_FILE_NAME);

    let configuration = File::open(configuration_path)
      // FIXME(error presentation): on invalid config, it will appear as though there is config with no veins.
      .and_then(|mut x| Configuration::read_configuration(&mut x))
      .unwrap_or_default();

    let veins = Veins::from_configuration(&configuration);
    // FIXME: only one (the first) Vein is taken
    let note_graph_ui = veins
      .iter()
      .next()
      .map(|(_, vein)| NoteGraphUi::new(Rc::clone(vein)));

    Self {
      basalt_dirs,
      veins,
      configuration,
      note_graph_ui,
    }
  }

}

impl eframe::App for BasaltApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    use egui::{CentralPanel, Color32, RichText};

    CentralPanel::default().show(ctx, |ui| {
      if ui.input(|input| input.key_pressed(egui::Key::R)) {
        self.read_configuration().expect("FIXME");
        let veins = Veins::from_configuration(&self.configuration);
        // FIXME: only one (the first) Vein is taken
        let note_graph_ui = veins
          .iter()
          .next()
          .map(|(_, vein)| NoteGraphUi::new(Rc::clone(vein)));

        self.note_graph_ui = note_graph_ui;
        log::info!("bruh");
      }

      if let Some(ref mut note_graph_ui) = &mut self.note_graph_ui {
        note_graph_ui.ui(ui);
      } else {
        const ERROR_TEXT: &str = r#"
Please create a file ~/.config/basalt/basalt.json and add
{
  "veins": ["/path/to/your/notes/folder"]
}
to it (must be an absolute path)
        "#;
        ui.label(RichText::new(ERROR_TEXT.trim()).color(Color32::RED));
      }
    });
  }
}
