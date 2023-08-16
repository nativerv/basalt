use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

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
  pub fn new_native(path: &Path) -> Self {
    use walkdir::WalkDir;

    let notes = WalkDir::new(path)
      .into_iter()
      .filter_map(Result::ok)
      .filter(|entry| entry.path().is_file())
      .map(|entry| {
        use crate::lib::path::PathExt;
        let prefix = path.canonicalize_unchecked();
        let note_path = entry.path().canonicalize_unchecked();
        let note_contents = std::fs::read_to_string(&note_path)
          // FIXME: (file read) probably should do something about the error
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

    Self {
      kind: Kind::Native {
        path: path.to_owned(),
      },
      notes,
    }
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
      Native { .. } => {
        self.notes.get(name.borrow()).map(String::as_str)
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
  fn vein_native__iter___gets_all_files() {
    let vein = Vein::new_native(Path::new("./tests/notes"));

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
  }
  #[test]
  fn vein_native__get_note___gets_file_contents() {
    let vein = Vein::new_native(Path::new("./tests/notes"));

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
  }
}
