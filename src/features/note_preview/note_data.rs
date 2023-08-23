
use pulldown_cmark::{Options, Parser};

use crate::lib::{
  images_cache::ImagesCache,
  publisher::{Publisher, Subscriber},
};

#[derive(Default)]
pub struct NoteData {
  text: String,
  pub images_cache: ImagesCache,
  subscribers: Vec<Subscriber>,
}

impl NoteData {
  pub fn new(markdown_input: String) -> Self {
    Self {
      text: markdown_input,
      images_cache: Default::default(),
      subscribers: Vec::new(),
    }
  }
  pub fn parsing_note(&self) -> Parser {
    Parser::new_ext(self.text.as_str(), Options::all())
  }

  pub fn set_text(&mut self, text: String) {
    self.text = text; 
    self.notify_all();
  }
}

impl Publisher for NoteData 
{
  fn subscribe(&mut self, subscriber: Subscriber) -> usize {
    self.subscribers.push(subscriber);
    self.subscribers.len() - 1
  }

  fn unsubscribe(&mut self, subscription: usize) {
    let _ = self.subscribers.remove(subscription);
  }

  fn notify_all(&mut self) {
    for item in self.subscribers.iter() {
      item();
    }
  }
}

#[cfg(test)]
mod test {
  use super::NoteData;

  #[test]
  fn parsing_note() {
    let note: NoteData = NoteData { text: "Hello world, [this](link.md) is a complicated *very simple* example.\n## Heading 2\n# Heading 1\n".to_string(), images_cache: Default::default(), subscribers: Vec::new() };
    note.parsing_note();
  }
}
