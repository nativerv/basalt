use eframe::egui;

use crate::features::configuration::Configuration;
use crate::features::note_graph::NoteGraphUi;
use crate::features::veins::{Veins, VeinId};
use directories::ProjectDirs;
use std::fs::File;
use std::rc::Rc;
use std::io;
use egui::{Event, Key};

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

impl BasaltApp {
  const CONFIG_FILE_NAME: &str = "basalt.json";

  fn read_configuration(&mut self) -> io::Result<()> {
    let configuration_path = self
      .basalt_dirs
      .config_dir()
      .join(Self::CONFIG_FILE_NAME);
    let configuration = File::open(configuration_path)
      // FIXME(error presentation): on invalid config, it will appear as though there is config with no veins.
      .and_then(|mut x| Configuration::read_configuration(&mut x))
      .unwrap_or_default();
    self.configuration = configuration;

    Ok(())
  }

  fn reload(&mut self) -> io::Result<()> {
    self.read_configuration()?;
    self.veins = Veins::from_configuration(&self.configuration)?;

    Ok(())
  }
}

impl BasaltApp {
  fn prev_vein(&mut self) {
    let prev_vein_id = self.veins
      .iter()
      .rev()
      .skip_while(|(&ref id, ..)| Some(id) != self.current_vein.as_ref())
      .map(|(id, ..)| id.clone())
      .nth(1)
    ;
    self.current_vein = prev_vein_id.or_else(|| self.current_vein.clone());
  }

  fn next_vein(&mut self) {
    let next_vein_id = self.veins.iter()
      .skip_while(|(&ref id, ..)| Some(id) != self.current_vein.as_ref())
      .map(|(id, ..)| id.clone())
      .nth(1)
    ;
    self.current_vein = next_vein_id.or_else(|| self.current_vein.clone());
  }
}

impl Default for BasaltApp {
  fn default() -> Self {
    // WARNING(portability): mobile

    // FIXME: maybe this should be global? Or fork the `directories` and add constructor into
    // custom dirs for `BasaltApp:;from_configuration` and tests?
    let basalt_dirs = ProjectDirs::from("com", "basalt", "basalt")
      .unwrap_or_else(|| {
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

    let veins = Veins::from_configuration(&configuration)
      .expect("FIXME (remove default)");

    let current_vein = veins
      .iter()
      .next()
      .map(|(vein_id, ..)| vein_id)
      .cloned()
    ;

    // FIXME: only one (the first) Vein is taken
    let note_graph_ui = current_vein
      .as_ref()
      .and_then(|vein_id| veins.get_vein(vein_id))
      .as_ref()
      .map(Rc::clone)
      .map(NoteGraphUi::new)
    ;

    Self {
      basalt_dirs,
      veins,
      current_vein,
      configuration,
      note_graph_ui,
    }
  }
}

struct VeinSelectionUi {
  veins: Vec<VeinId>,
  vein_id_list: Vec<VeinId>,
}

impl eframe::App for BasaltApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    use egui::{CentralPanel, Color32, RichText};

    CentralPanel::default().show(ctx, |ui| {
      ctx.set_style(egui::Style {
        debug: egui::style::DebugOptions {
          debug_on_hover: true,
          ..Default::default()
        },
        ..Default::default()
      });
      if ui.input(|input| input.key_pressed(egui::Key::R)) {
        self.reload().expect("FIXME");
        // FIXME: only one (the first) Vein is taken
        self.note_graph_ui = self.veins
          .iter()
          .next()
          .map(|(_, vein)| NoteGraphUi::new(Rc::clone(vein)));
      }

      ui.input(|input| {
        for event in input.events.iter() {
          match event {
            Event::Key { key: Key::K, pressed: true, .. } => self.prev_vein(),
            Event::Key { key: Key::J, pressed: true, .. } => self.next_vein(),
            _ => {},
          }
        }
      });

      if let Some(ref mut note_graph_ui) = &mut self.note_graph_ui {
        ui.vertical(|ui| {
          for (vein_id, ..) in self.veins.iter() {
            let is_selected = self.current_vein
              .as_ref()
              .map(|self_vein_id| self_vein_id == vein_id)
              .unwrap_or(false);
            if ui.selectable_label(is_selected, &**vein_id).clicked() {
              self.current_vein = Some(vein_id.clone());
            }
          }
        });
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
