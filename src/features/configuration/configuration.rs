use egui::Color32;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone,)]
struct Configuration {
  pub include: Vec<PathBuf>,
  pub background_color: Color32,
  pub foreground_color: Color32,
  pub primary_color: Color32,
  pub secondary_color: Color32,
}

impl Default for Configuration {
  fn default() -> Self {
    Self {
      include: vec![],
      background_color: Color32::from_rgb(0, 0, 0),
      foreground_color: Color32::from_rgb(255, 255, 255),
      primary_color: Color32::from_rgb(255, 255, 255),
      secondary_color: Color32::from_rgb(255, 255, 255),
    }
  }
}

impl Configuration {
  pub fn read_configuration_from_file(path: &PathBuf) -> Configuration {
    let mut file = File::open(path).expect("Could not open file");
    let mut content = String::new();
    file
      .read_to_string(&mut content)
      .expect("Could not read file");
    let mut configuration: Configuration =
      serde_json::from_str(&content).expect("JSON was not well-formatted");
    for included_file in configuration.clone().include.iter() {
      configuration = Configuration::read_configuration_from_file(included_file)
    }
    configuration
  }

  pub fn write_configuration_to_file(path: PathBuf, configuration: &Configuration) {
    let mut file = File::create(path).expect("Could not create file");
    let content = serde_json::to_string(&configuration).expect("Could not serialize configuration");
    file
      .write_all(content.as_bytes())
      .expect("Could not write file");
  }
}

mod test {
  use egui::Color32;
  use std::path::PathBuf;
  use crate::features::configuration::configuration::Configuration;
  

  #[test]
  fn write() {
    Configuration::write_configuration_to_file(
      PathBuf::from("config.json"),
      &Configuration::default(),
    );
  }

  #[test]
  fn write_and_read() {
    let start_config: Configuration = Configuration {
      include: vec![],
      background_color: Color32::from_rgb(0, 0, 0),
      foreground_color: Color32::from_rgb(0, 0, 0),
      primary_color: Color32::from_rgb(0, 0, 0),
      secondary_color: Color32::from_rgb(0, 0, 0),
    };
    Configuration::write_configuration_to_file(PathBuf::from("config.json"), &start_config);
    let readed_config = Configuration::read_configuration_from_file(&PathBuf::from("config.json"));
    assert_eq!(start_config, readed_config);
  }

  #[test]
  fn write_and_read_multiple_file()
  {
    let start_config: Configuration = Configuration {
      include: vec![PathBuf::from("second_file.json")],
      background_color: Color32::from_rgb(0, 0, 0),
      foreground_color: Color32::from_rgb(0, 0, 0),
      primary_color: Color32::from_rgb(0, 0, 0),
      secondary_color: Color32::from_rgb(0, 0, 0),
    };
    let second_config: Configuration = Configuration {
      include: vec![],
      background_color: Color32::from_rgb(255, 255, 255),
      foreground_color: Color32::from_rgb(0, 0, 0),
      primary_color: Color32::from_rgb(255, 0, 255),
      secondary_color: Color32::from_rgb(0, 255, 0),
    };
    Configuration::write_configuration_to_file(PathBuf::from("config.json"), &start_config);
    Configuration::write_configuration_to_file(PathBuf::from("second_file.json"), &second_config);
    let readed_config = Configuration::read_configuration_from_file(&PathBuf::from("config.json"));
    assert_eq!(second_config, readed_config);
  }
}
