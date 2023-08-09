use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};

type NoteId = String;
type Note = String;

/// Veins are a name for Basalt's note repositories.
/// Example: ~/Documents/personal-notes
/// Example: ~/Documents/work-notes
pub enum Vein {
  Native {
    path: PathBuf,
    notes: HashMap<NoteId, Note>,
  },
  #[allow(dead_code)]
  Web {},
  #[allow(dead_code)]
  Remote {},
}

impl Vein {
  pub fn new_native(path: &Path) -> Self {
    use std::io::Read;
    use walkdir::WalkDir;

    let notes = WalkDir::new(path)
      .into_iter()
      .filter_map(|maybe_entry| maybe_entry.ok())
      .filter(|entry| entry.path().is_file())
      .map(|entry| {
        let path = entry.path();
        let contents = File::options()
          .read(true)
          .open(path)
          .map(|mut file| {
            let mut buf = Note::new();
            // TODO: (file read) (incorrect unicode) probably should do something about the error
            // (otherwise files that failed to be parsed will appear empty
            // without notice)
            file
              .read_to_string(&mut buf)
              .map(|_| buf)
              .unwrap_or_default()
          })
          // TODO: (file read) probably should do something about the error
          // (otherwise files that failed to be read will appear empty
          // without notice)
          .unwrap_or_default();
        // FIXME: figure out what's all this (path to string) bullshit is about
        (
          path.as_os_str().to_str().unwrap_or_default().to_owned(),
          contents,
        )
      })
      .collect::<HashMap<NoteId, Note>>();

    Self::Native {
      path: path.to_owned(),
      notes,
    }
  }

  fn iter<'a>(&'a self) -> Iter<'a> {
    use Vein::*;
    match self {
      Native { notes, .. } => Iter::Native {
        notes_iter: notes.iter(),
      },
      Web { .. } => unimplemented!(),
      Remote { .. } => unimplemented!(),
    }
  }

  fn get_note<Q>(&self, name: Q) -> Option<&'_ str>
  where
    Q: std::borrow::Borrow<str> + std::hash::Hash + std::cmp::Eq,
  {
    use Vein::*;
    match self {
      Native { notes, .. } => {
        // let path = path.to_str().map(|str| std::borrow::Cow::from(str)).unwrap_or_else(|| path.to_string_lossy());
        notes
          .get(name.borrow())
          .map(|string_ref| string_ref.as_str())
      }
      Web {} => unimplemented!(),
      Remote {} => unimplemented!(),
    }
  }
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
  fn vein_native__notes___gets_all_files() {
    let vein = Vein::new_native(Path::new("./tests/notes"));

    let expected = [
      "./tests/notes/basalt.md",
      "./tests/notes/bio.md",
      "./tests/notes/build-tool.md",
      "./tests/notes/cargo.md",
      "./tests/notes/diary/20230803101243-walkdir.md",
      "./tests/notes/index.md",
      "./tests/notes/interests.md",
      "./tests/notes/internets.md",
      "./tests/notes/me.md",
      "./tests/notes/note-taking-software.md",
      "./tests/notes/programming-language.md",
      "./tests/notes/programming.md",
      "./tests/notes/rust.md",
      "./tests/notes/software.md",
      "./tests/notes/wikipedia.md",
    ]
    .as_slice()
    .iter();

    let mut note_paths = vein
      .iter()
      .map(|(note_id, ..)| note_id)
      .collect::<Vec<&str>>();
    note_paths.sort();

    assert!(expected.eq(note_paths.iter()));
  }
  #[test]
  fn vein_native__notes___gets_file_contents() {
    let vein = Vein::new_native(Path::new("./tests/notes"));

    let note_contents = vein.get_note("./tests/notes/basalt.md").unwrap();

    let expected = r#"# Basalt is an igneous rock.

#[rust](rust.md) #[note-taking-software](note-taking-software.md) #[zettelkasten](zettelkasten.md) #[cross-platform](cross-platform.md)

*And* it's a [mind-~~mine~~map application](https://github.com/nativerv/basalt).
"#;

    assert_eq!(expected, note_contents);

    let expected = r#"# `walkdir`: a crate that traverses filesystem recursively

#[cross-platform](cross-platform.md) #[rust](rust.md) #[crate](crate.md)
"#;
    let note_contents = vein
      .get_note("./tests/notes/diary/20230803101243-walkdir.md")
      .unwrap();
    assert_eq!(expected, note_contents);
  }
}
