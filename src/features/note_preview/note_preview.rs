use pulldown_cmark::{Event, Parser, Options};

#[derive(Default)]
pub struct NotePreview {
  pub markdown_input: String,
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
    let note: NotePreview = NotePreview { markdown_input: "Hello world, [this](link.md) is a complicated *very simple* example.\n## Heading 2\n# Heading 1\n".to_string()};
    note.parsing_note();
  }
}
