use crate::features::configuration::Configuration;
use crate::features::note_graph::NoteGraphUi;
use crate::features::veins::{Vein, VeinId, VeinSelectionUi, Veins};
use directories::ProjectDirs;
use egui::{CentralPanel, Event, Key};
use std::cell::RefCell;
use std::fs::File;
use std::io;
use std::rc::Rc;

/// Global Basalt state
pub enum BasaltApp {
  Ok {
    basalt_dirs: ProjectDirs,
    configuration: Configuration,
    /// A list of veins known to the this Basalt instance.
    veins: Veins,
    current_vein: Option<VeinId>,
    note_graph_ui: Option<NoteGraphUi>,
  },
  ConfigurationError {
    message: String,
    basalt_dirs: ProjectDirs,
    note_graph_ui: NoteGraphUi,
  },
  NoVeins {
    message: String,
    basalt_dirs: ProjectDirs,
    note_graph_ui: NoteGraphUi,
  },
  BasaltDirsError {
    message: String,
    note_graph_ui: NoteGraphUi,
  },
}

impl Default for BasaltApp {
  fn default() -> Self {
    // FIXME(portability): mobile

    let Some(basalt_dirs) = ProjectDirs::from("com", "basalt", "basalt") else {
      // TODO: also have a way to specify dirs as cli args or env vars
      log::error!("initializing basalt: CRITICAL: could not retrieve valid home directory path for your OS: required for configuration dir");
      let message = String::from("Critical error: HOME directory could not be retrieved from the operating system. Either you have critical problems with your system or have launched Basalt weirdly.");
      return Self::BasaltDirsError {
        message,
        note_graph_ui: NoteGraphUi::new(Rc::new(RefCell::new(
          Vein::new_native_temp_vein().expect("FIXME"),
        ))),
      };
    };

    let configuration_path = basalt_dirs.config_dir().join(Self::CONFIG_FILE_NAME);
    let configuration = match File::open(&configuration_path)
      .and_then(|mut x| Configuration::read_configuration(&mut x))
    {
      Ok(configuration) => configuration,
      Err(error) => {
        #[rustfmt::skip]
        let message = match error.kind() {
          // TODO: FIXME: custom error type (io error kinds may overlap from different sources)
          io::ErrorKind::InvalidData => {
            log::error!("initializing basalt: could not parse configuration file");
            log::error!("the exact error is: {error:#?}");
            format!("Could not parse your Basalt config. Please go to '{path}' and fix your config.", path = configuration_path.display())
          },
          io::ErrorKind::UnexpectedEof => {
            log::error!("initializing basalt: unexpected end of configuration file (unexpected EOF)");
            log::error!("the exact error is: {error:#?}");
            format!("Could not parse your Basalt config: unexpected end file. Please go to '{path}' and fix your config.", path = configuration_path.display())
          },
          io::ErrorKind::NotFound => {
            log::error!("initializing basalt: configuration file ('{path}') does not exist", path = configuration_path.display());
            log::error!("the exact error is: {error:#?}");
            format!("Basalt configuration file does not exist. Please create and populate the file '{path}'.", path = configuration_path.display())
          },
          io::ErrorKind::PermissionDenied => {
            log::error!("initializing basalt: permission denied for configuration file ('{path}')", path = configuration_path.display());
            log::error!("the exact error is: {error:#?}");
            format!("Permission denied while trying to read Basalt configuration file. Please ensure that '{path}' is accessible to Basalt.", path = configuration_path.display())
          },
          _ => {
            log::error!("initializing basalt: unexpected error");
            log::error!("the exact error is: {error:#?}");
            format!("Unexpected error while reading Basalt config. Please ensure that path '{path}' exists and is accessible to Basalt.", path = configuration_path.display())
          },
        };
        return Self::ConfigurationError {
          message,
          basalt_dirs,
          note_graph_ui: NoteGraphUi::new(Rc::new(RefCell::new(
            Vein::new_native_temp_vein().expect("FIXME"),
          ))),
        };
      }
    };

    // Some veins can exist, some can error out...
    let veins = Veins::from_configuration(&configuration);

    if veins.iter().count() < 1 {
      log::error!("initializing basalt: no veins specified");
      // FIXME: windows directory
      let message = format!(
        r#"
        You haven't specified any Veins in your configuration. Basalt calls it's note directories *Veins*.
        Please go to your Basalt configuration file ('{path}') and add paths to your note directories as so:
        ```
        {{
          "veins": ["/path/to/your/notes/folder", "/path/to/another/one"]
        }}
        ```
        to it (must be an absolute path)
        "#,
        path = configuration_path.display()
      );

      return Self::NoVeins {
        message,
        basalt_dirs,
        note_graph_ui: NoteGraphUi::new(Rc::new(RefCell::new(
          Vein::new_native_temp_vein().expect("FIXME"),
        ))),
      };
    }

    // FIXME: only one (the first) Vein is taken, should probably persist selected
    let current_vein = veins.iter().next().map(|(vein_id, ..)| vein_id.clone());
    // TODO: clippy suggests a fix (only on nightly) that produces an error
    #[allow(clippy::useless_asref)]
    let note_graph_ui = current_vein
      .as_ref()
      .and_then(|vein_id| veins.get_vein(vein_id))
      .and_then(|maybe_vein| {
        maybe_vein
          .as_ref()
          .map(Rc::clone)
          .map(NoteGraphUi::new)
          .ok()
      });

    Self::Ok {
      basalt_dirs,
      veins,
      current_vein,
      configuration,
      note_graph_ui,
    }
  }
}

impl BasaltApp {
  const CONFIG_FILE_NAME: &'static str = "basalt.json";

  fn reload(&mut self) {
    *self = Self::default();
  }

  fn prev_vein(&mut self) {
    if let Self::Ok {
      veins,
      current_vein,
      ..
    } = self
    {
      let prev_vein_id = veins
        .iter()
        .rev()
        .skip_while(|(id, ..)| Some(id) != current_vein.as_ref().as_ref())
        .map(|(id, ..)| id.clone())
        .nth(1);
      *current_vein = prev_vein_id.or_else(|| current_vein.clone());
    }
  }

  fn next_vein(&mut self) {
    if let Self::Ok {
      veins,
      current_vein,
      ..
    } = self
    {
      let next_vein_id = veins
        .iter()
        .skip_while(|(id, ..)| Some(id) != current_vein.as_ref().as_ref())
        .map(|(id, ..)| id.clone())
        .nth(1);
      *current_vein = next_vein_id.or_else(|| current_vein.clone());
    };
  }

  fn handle_global_keys(&mut self, ctx: &egui::Context) {
    use egui::Modifiers;

    let mods = ctx.input(|input| input.modifiers);
    #[rustfmt::skip]
    match mods {
      Modifiers { alt: true, .. } => ctx.set_debug_on_hover(true),
      Modifiers { alt: false, .. } => ctx.set_debug_on_hover(false),
    }

    // PERF: `Vec` clone each frame
    let events = ctx.input(|input| input.events.clone());
    for event in events.iter() {
      #[rustfmt::skip]
      match event {
        Event::Key { key: Key::K, pressed: true, .. } => self.prev_vein(),
        Event::Key { key: Key::J, pressed: true, .. } => self.next_vein(),
        Event::Key { key: Key::R, pressed: true, .. } => self.reload(),
        _ => {},
      }
    }
  }
}

impl eframe::App for BasaltApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    self.handle_global_keys(ctx);

    CentralPanel::default().show(ctx, |ui| {
      match self {
        Self::Ok {
          basalt_dirs: _,
          configuration: _,
          veins,
          current_vein,
          note_graph_ui,
        } => {
          ui.vertical(|ui| {
            ui.add(VeinSelectionUi::new(&*veins, current_vein));
          });
          ui.vertical(|ui| {
            if let Some(note_graph_ui) = note_graph_ui.as_mut() {
              note_graph_ui.ui(ui)
            }
          });
        }
        _error => unimplemented!(),
      };
    });
  }
}
