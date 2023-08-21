use pulldown_cmark::{Options, Parser};

use crate::lib::images_cache::ImagesCache;

#[derive(Default)]
pub struct NoteData {
  pub markdown_input: String,
  pub images_cache: ImagesCache,
}

impl NoteData {
  pub fn parsing_note(&self) -> Parser {
    Parser::new_ext(self.markdown_input.as_str(), Options::all())
  }
}

#[cfg(test)]
mod test {
  use super::NoteData;

  #[test]
  fn parsing_note() {
    let note: NoteData = NoteData { markdown_input: "Hello world, [this](link.md) is a complicated *very simple* example.\n## Heading 2\n# Heading 1\n".to_string(), images_cache: Default::default() };
    note.parsing_note();
  }
}
