use egui::Color32;
use serde::{Deserialize, Serialize};
use serde_json::{self, Map, Value};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub struct Configuration {
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
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
    let s = serde_json::to_string(&value)
      .expect("invariant: what cound possibly happen that a `Map` will not serialize?");
    serde_json::from_str(&s).expect("invariant: we just got that `str` from `to_string`")
  }
}

impl Configuration {
  //TODO: replace json to conf format with include ordering
  #[cfg_attr(not(test), expect(unused))]
  pub fn read_configuration(readable_content: &mut impl Read) -> io::Result<Self> {
    let mut content = String::new();
    readable_content.read_to_string(&mut content)?;
    let mut configuration_map: Map<String, Value> = serde_json::from_str(&content)?;
    let temp_config: Self = serde_json::from_str(&content)?;
    for included_file in temp_config.include.iter() {
      Self::read_configuration_inside(&mut File::open(included_file)?, &mut configuration_map)?;
    }
    Ok(Self::from(&configuration_map))
  }

  fn read_configuration_inside(
    readable_content: &mut impl Read,
    configuration: &mut Map<String, Value>,
  ) -> io::Result<()> {
    let mut content = String::new();
    readable_content.read_to_string(&mut content)?;
    let current_configuration: Map<String, Value> = serde_json::from_str(&content)?;
    for (key, val) in current_configuration.into_iter() {
      // Modify or insert
      if let Some(old) = configuration.get_mut(&key) {
        *old = val;
      } else {
        configuration.insert(key, val);
      }
      // TODO: Entry API: find out how to work out of clones
      // (`val` attempted to be moved into 2 places without them)
      // The Entry API is really beautiful but as it stands here it's
      // pretty much unusable.
      // configuration
      //   .entry(&key)
      //   .and_modify(|e| *e = val.clone())
      //   .or_insert_with(|| val.clone());
    }
    let included_files = Self::from(&*configuration).include;
    for included_file in included_files.iter() {
      Self::read_configuration_inside(&mut File::open(included_file)?, configuration)?;
    }

    Ok(())
  }
}

impl Configuration {
  // Every `expect(unused)` fn has to be in it's own impl block because of a Rust bug:
  // <https://github.com/rust-lang/rust/issues/114416>
  #[cfg_attr(not(test), expect(unused))]
  pub fn write_configuration(&self, writable_content: &mut impl Write) -> io::Result<()> {
    let content = serde_json::to_string_pretty(self)?;
    writable_content.write_all(content.as_bytes())
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use std::fs::File;

  #[test]
  fn read() {
    let read_config = Configuration::read_configuration(
      &mut File::open("tests/configuration/config.json").expect("Could not open file"),
    );
    assert_eq!(Configuration::default(), read_config.unwrap());
  }

  #[test]
  fn read_multiple_files() {
    let expected_config = Configuration {
      include: vec![],
      foreground_color: Color32::from_rgb(255, 255, 255),
      background_color: Color32::from_rgb(255, 255, 255),
      primary_color: Color32::from_rgb(255, 0, 255),
      secondary_color: Color32::from_rgb(0, 167, 0),
    };
    let read_config = Configuration::read_configuration(
      &mut File::open("tests/configuration/first_config.json").expect("Could not open file"),
    );
    assert_eq!(expected_config, read_config.unwrap());
  }

  #[test]
  fn read_multiple_files_with_partial_initial_config() {
    let expected_config = Configuration {
      include: vec![],
      foreground_color: Color32::from_rgb(255, 255, 255),
      background_color: Color32::from_rgb(255, 255, 255),
      primary_color: Color32::from_rgb(255, 0, 255),
      secondary_color: Color32::from_rgb(0, 167, 0),
    };
    let read_config = Configuration::read_configuration(
      &mut File::open("tests/configuration/first_config_partial.json")
        .expect("Could not open file"),
    );
    assert_eq!(expected_config, read_config.unwrap());
  }

  #[test]
  fn write_and_read() {
    let expected = Configuration {
      include: vec![],
      background_color: Color32::from_rgb(0, 0, 0),
      foreground_color: Color32::from_rgb(0, 0, 0),
      primary_color: Color32::from_rgb(0, 0, 0),
      secondary_color: Color32::from_rgb(0, 0, 0),
    };
    crate::lib::test::with_test_dir(|temp_dir| {
      let first_config_file = temp_dir.join("first_config_file.json");
      expected
        .write_configuration(
          &mut File::create(&first_config_file).expect("Could not open file for write"),
        )
        .unwrap();

      let actual =
        Configuration::read_configuration(&mut File::open(&first_config_file).unwrap()).unwrap();
      assert_eq!(expected, actual);

      Ok(())
    })
    .unwrap();
  }
}
