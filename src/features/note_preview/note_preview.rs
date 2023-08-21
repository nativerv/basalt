use pulldown_cmark::{Parser, Options};

use crate::lib::images_cache::ImagesCache;

#[derive(Default)]
pub struct NotePreview {
  pub markdown_input: String,
  pub images_cache: ImagesCache,

}

impl NotePreview {
  pub fn parsing_note(&self) -> Parser {
    Parser::new_ext(self.markdown_input.as_str(), Options::all())
  }
}

#[cfg(test)]
mod test {
  use super::NotePreview;

  #[test]
  fn parsing_note() {
    let note: NotePreview = NotePreview { markdown_input: "Hello world, [this](link.md) is a complicated *very simple* example.\n## Heading 2\n# Heading 1\n".to_string(), images_cache: Default::default() };
    note.parsing_note();
  }
}
