use crate::lib::path::PathExt;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

/// Represents a data type that can be stored in a Vein's config storage.
pub trait Store {
  type Error: Into<io::Error>;

  fn vein_config_name() -> &'static str;
  fn serialize(&self) -> Result<String, Self::Error>;
  fn deserialize(s: impl AsRef<str>) -> Result<Self, Self::Error>
  where
    Self: Sized;
}

type NoteId = String;
type Note = String;
type Notes = HashMap<NoteId, Note>;

/// Veins are a name for Basalt's note repositories.
/// Example: ~/Documents/personal-notes
/// Example: ~/Documents/work-notes
#[derive(Debug)]
pub enum Kind {
  Native {
    path: PathBuf,
  },
  #[allow(dead_code)]
  Web {},
  #[allow(dead_code)]
  Remote {},
}

#[derive(Debug)]
pub struct Vein {
  kind: Kind,
  notes: Notes,
}

/// Public methods
impl Vein {
  const CONFIG_DIRECTORY: &str = ".basalt";

  pub fn new_native(path: &Path) -> io::Result<Self> {
    use walkdir::WalkDir;

    let path = path.canonicalize_unchecked();

    // Check that path exists & a dir, else return.
    path.is_dir().then_some(()).ok_or_else(|| {
      io::Error::new(
        io::ErrorKind::InvalidInput,
        format!(
          "Invalid Vein directory: '{}'",
          path.canonicalize_unchecked().display()
        ),
      )
    })?;

    let notes = WalkDir::new(&path)
      .into_iter()
      .filter_map(Result::ok)
      .filter(|entry| entry.path().is_file())
      .map(|entry| {
        let prefix = path.canonicalize_unchecked();
        let note_path = entry.path().canonicalize_unchecked();
        let note_contents = std::fs::read_to_string(&note_path)
          // FIXME: handle silent error (read notes)
          // (otherwise files that failed to be read will appear empty
          // without notice)
          .unwrap_or_default();

        // FIXME: this error also
        let note_id = note_path
          .strip_prefix(&prefix)
          .expect("can strip prefix")
          .to_str()
          .expect("expected path to be a valid unicode")
          .to_owned();

        (note_id, note_contents)
      })
      .collect::<Notes>();

    Ok(Self {
      kind: Kind::Native {
        path: path.to_owned(),
      },
      notes,
    })
  }

  pub fn iter<'a>(&'a self) -> Iter<'a> {
    use Kind::*;
    match &self.kind {
      Native { .. } => Iter::Native {
        notes_iter: self.notes.iter(),
      },
      Web { .. } => unimplemented!(),
      Remote { .. } => unimplemented!(),
    }
  }

  pub fn get_note<Q>(&self, name: Q) -> Option<&'_ str>
  where
    Q: std::borrow::Borrow<str>,
  {
    use Kind::*;
    match &self.kind {
      Native { .. } => self.notes.get(name.borrow()).map(String::as_str),
      Web { .. } => unimplemented!(),
      Remote { .. } => unimplemented!(),
    }
  }

  pub fn read_config_value<T>(&self) -> io::Result<T>
  where
    T: Store<Error = io::Error> + serde::de::DeserializeOwned,
  {
    use Kind::*;
    match &self.kind {
      Native { path, .. } => {
        let config_file_name = Path::new(<T as Store>::vein_config_name());
        let text =
          std::fs::read_to_string(path.join(Vein::CONFIG_DIRECTORY).join(config_file_name))?;
        Ok(Store::deserialize(text)?)
      }
      Web { .. } => unimplemented!(),
      Remote { .. } => unimplemented!(),
    }
  }
  pub fn write_config_value<T>(&self, value: &T) -> io::Result<()>
  where
    T: Store<Error = io::Error> + serde::Serialize,
  {
    use Kind::*;
    match &self.kind {
      Native { path, .. } => {
        let config_file_name = Path::new(<T as Store>::vein_config_name());
        let json = Store::serialize(value)?;
        let config_file_path = path.join(Vein::CONFIG_DIRECTORY).join(config_file_name);
        let config_file_dir_path = &config_file_path
          .parent()
          .expect("invariant: should be at least Vein's root folder, because it's joined with `Vein::CONFIG_DIRECTORY`");
        std::fs::create_dir_all(config_file_dir_path)?;
        Ok(std::fs::write(config_file_path, json)?)
      }
      Web { .. } => unimplemented!(),
      Remote { .. } => unimplemented!(),
    }
  }
}

/// Private methods
impl Vein {
  // fn normalize_path() -> Option<&Path> {
  //
  // }
}

pub enum Iter<'a> {
  Native {
    notes_iter: std::collections::hash_map::Iter<'a, NoteId, Note>,
  },
  #[allow(dead_code)]
  Web {},
  #[allow(dead_code)]
  Remote {},
}

impl<'a> Iterator for Iter<'a> {
  type Item = (&'a str, &'a str);

  fn next(&mut self) -> Option<Self::Item> {
    use Iter::*;
    match self {
      Native { notes_iter, .. } => {
        let (note_id, note) = notes_iter.next()?;
        Some((note_id.as_str(), note.as_str()))
      }

      Web {} => unimplemented!(),
      Remote {} => unimplemented!(),
    }
  }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
  use super::*;

  #[test]
  fn vein_native__iter___gets_all_files() -> io::Result<()> {
    let vein = Vein::new_native(Path::new("./tests/notes"))?;

    let expected = [
      "basalt.md",
      "bio.md",
      "build-tool.md",
      "cargo.md",
      "diary/20230803101243-walkdir.md",
      "index.md",
      "interests.md",
      "internets.md",
      "me.md",
      "note-taking-software.md",
      "programming-language.md",
      "programming.md",
      "rust.md",
      "software.md",
      "wikipedia.md",
    ]
    .as_slice();

    let mut note_paths = vein
      .iter()
      .map(|(note_id, ..)| note_id)
      .collect::<Vec<&str>>();
    note_paths.sort();

    assert!(
      expected.iter().eq(note_paths.iter()),
      "expected: {:#?}, was: {:#?}",
      expected.iter(),
      note_paths.iter()
    );

    Ok(())
  }
  #[test]
  fn vein_native__get_note___gets_file_contents() -> io::Result<()> {
    let vein = Vein::new_native(Path::new("./tests/notes"))?;

    dbg!(&vein);
    let note_contents = vein.get_note("basalt.md").unwrap();

    let expected = r#"# Basalt is an igneous rock.

#[rust](rust.md) #[note-taking-software](note-taking-software.md) #[zettelkasten](zettelkasten.md) #[cross-platform](cross-platform.md)

*And* it's a [mind-~~mine~~map application](https://github.com/nativerv/basalt).
"#;

    assert_eq!(expected, note_contents);

    let expected = r#"# `walkdir`: a crate that traverses filesystem recursively

#[cross-platform](cross-platform.md) #[rust](rust.md) #[crate](crate.md)
"#;
    let note_contents = vein.get_note("diary/20230803101243-walkdir.md").unwrap();
    assert_eq!(expected, note_contents);

    Ok(())
  }

  #[test]
  fn vein_native__new_native___errors_when_invalid() -> io::Result<()> {
    crate::lib::test::with_test_dir(|tmp_dir| {
      let vein = Vein::new_native(&tmp_dir.join("nonexistent"));
      assert_eq!(vein.unwrap_err().kind(), io::ErrorKind::InvalidInput);
      Ok(())
    })
  }

  /// Mock Store impl for testing purposes
  #[derive(serde::Deserialize, serde::Serialize, PartialEq, Eq, Debug)]
  struct Data(String);
  impl Store for Data {
    type Error = io::Error;

    fn vein_config_name() -> &'static str {
      "data/data.json"
    }
    fn serialize(&self) -> Result<String, io::Error> {
      Ok(serde_json::to_string(self)?)
    }
    fn deserialize(s: impl AsRef<str>) -> Result<Self, io::Error> {
      Ok(serde_json::from_str(s.as_ref())?)
    }
  }

  #[test]
  fn vein_native__read_config_value() -> io::Result<()> {
    let expected = Data(String::from("test"));

    let actual = crate::lib::test::with_test_dir(|tmp_dir| {
      let vein = Vein::new_native(&tmp_dir)?;
      let expected_file_path = tmp_dir
        .join(Vein::CONFIG_DIRECTORY)
        .join("data")
        .join("data.json");
      std::fs::create_dir_all(expected_file_path.parent().expect("parent"))?;
      std::fs::write(expected_file_path, expected.serialize()?.as_bytes())?;
      Ok(vein.read_config_value::<Data>().expect("expect can read"))
    })?;

    assert_eq!(expected, actual);
    Ok(())
  }

  #[test]
  fn vein_native__write_config_value() -> io::Result<()> {
    let expected = Data(String::from("test"));

    let actual = crate::lib::test::with_test_dir(|tmp_dir| {
      let vein = Vein::new_native(&tmp_dir)?;
      vein.write_config_value(&expected)?;
      let config_file_path = tmp_dir
        .join(Vein::CONFIG_DIRECTORY)
        .join("data")
        .join("data.json");
      assert!(config_file_path.is_file());
      Ok(<Data as Store>::deserialize(&std::fs::read_to_string(
        config_file_path,
      )?)?)
    })?;

    assert_eq!(expected, actual);
    Ok(())
  }
}
