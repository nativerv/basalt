use egui::Color32;
use serde::{Deserialize, Serialize};
use serde_json::{self, Map, Value};
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct Configuration {
  #[serde(default)]
  pub include: Vec<PathBuf>,
  #[serde(default)]
  pub background_color: Color32,
  #[serde(default)]
  pub foreground_color: Color32,
  #[serde(default)]
  pub primary_color: Color32,
  #[serde(default)]
  pub secondary_color: Color32,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
struct ConfigurationOptional {
  pub include: Option<Vec<PathBuf>>,
  pub background_color: Option<Color32>,
  pub foreground_color: Option<Color32>,
  pub primary_color: Option<Color32>,
  pub secondary_color: Option<Color32>,
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

impl From<&Map<String, Value>> for Configuration {
    fn from(value: &Map<String, Value>) -> Self {
      let s = serde_json::to_string(&value).expect("To string convert error");
      serde_json::from_str(&s).expect("From string convert error")
    }
}

impl Configuration {
  //TODO: replace json to conf format with include ordering 
  pub fn read_configuration(readable_content: &mut impl Read) -> Configuration {
    let mut content = String::new();
    readable_content
      .read_to_string(&mut content)
      .expect("Could not read file");
    let mut configuration_map: Map<String, Value> =
      serde_json::from_str(&content).expect("From string convert error");
    let temp_config:Configuration = serde_json::from_str(&content).expect("From string convert error");
    for included_file in temp_config.include.iter() {
      Configuration::read_configuration_inside(
        &mut File::open(&included_file).expect("Could not read file"),
        &mut configuration_map,
      )
    }
    Configuration::from(&configuration_map)
  }

  fn read_configuration_inside(
    readable_content: &mut impl Read,
    configuration: &mut Map<String, Value>,
  ) {
    let mut content = String::new();
    readable_content
      .read_to_string(&mut content)
      .expect("Could not read file");
    let current_configuration: Map<String, Value> =
      serde_json::from_str(&content).expect("JSON was not well-formatted");
    for (str, val) in current_configuration {
      *configuration.get_mut(&str).expect("Get mutable value error") = val;
    }
    let included_files = Configuration::from(&*configuration).include;
    for included_file in included_files.iter() {
      Configuration::read_configuration_inside(
        &mut File::open(&included_file).expect("Could not read file"),
        configuration,
      )
    }
  }

  pub fn write_configuration(writable_content: &mut impl Write, configuration: &Configuration) {
    let content = serde_json::to_string_pretty(&configuration).expect("Could not serialize configuration");
    writable_content
      .write_all(content.as_bytes())
      .expect("Could not write file");
  }
}

mod test {
  use crate::features::configuration::configuration::Configuration;
  use egui::Color32;
  use std::fs::File;
  use std::io::{Seek, SeekFrom};
  use std::path::PathBuf;

  #[test]
  fn read() {
    let readed_config = Configuration::read_configuration(
      &mut File::open("tests/config.json").expect("Could not open file"),
    );
    assert_eq!(Configuration::default(), readed_config);
  }

  #[test]
  fn read_multiple_file() {
    let expected_config: Configuration = Configuration {
      include: vec![],
      foreground_color: Color32::from_rgb(255, 255, 255),
      background_color: Color32::from_rgb(255, 255, 255),
      primary_color: Color32::from_rgb(255, 0, 255),
      secondary_color: Color32::from_rgb(0, 167, 0),
    };
    let read_config = Configuration::read_configuration(
      &mut File::open("tests/first_config.json").expect("Could not open file"),
    );
    assert_eq!(expected_config, read_config);
  }

  #[test]
  #[ignore = "interferes with other tests"]
  fn write_to_file() {
    Configuration::write_configuration(
      &mut File::create("tests/config_write.json").expect("Could not open file for write"),
      &Configuration::default(),
    );
  }

  #[test]
  #[ignore = "interferes with other tests"]
  fn write_and_read() {
    let start_config: Configuration = Configuration {
      include: vec![],
      background_color: Color32::from_rgb(0, 0, 0),
      foreground_color: Color32::from_rgb(0, 0, 0),
      primary_color: Color32::from_rgb(0, 0, 0),
      secondary_color: Color32::from_rgb(0, 0, 0),
    };
    Configuration::write_configuration(
      &mut File::create("tests/config_write.json").expect("Could not open file for write"),
      &start_config,
    );
    let readed_config = Configuration::read_configuration(
      &mut File::open("tests/config_write.json").expect("Could not open file"),
    );
    assert_eq!(start_config, readed_config);
  }

  #[test]
  #[ignore = "interferes with other tests"]
  fn write_and_read_multiple_file() {
    let start_config: Configuration = Configuration {
      include: vec![PathBuf::from("tests/second_config_write.json")],
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
    let mut config_file = File::options()
      .write(true)
      .read(true)
      .create(true)
      .open("tests/first_config_write.json")
      .expect("Could not open file for write");
    Configuration::write_configuration(&mut config_file, &start_config);
    Configuration::write_configuration(
      &mut File::create("tests/second_config_write.json").expect("Could not open file for write"),
      &second_config,
    );
    let _ = &mut config_file
      .seek(SeekFrom::Start(0))
      .expect("Could not seek file");
    let readed_config = Configuration::read_configuration(&mut config_file);
    assert_eq!(second_config, readed_config);
  }
}
