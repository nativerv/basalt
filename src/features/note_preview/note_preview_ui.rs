use std::{cell::RefCell, rc::Rc};

use egui::Ui;

use crate::ui::Note;

use super::NoteData;

pub struct NotePreviewUi {
  note: Note,
}

impl NotePreviewUi {
  pub fn new(note_data: Rc<RefCell<NoteData>>) -> Self {
    Self {
      note: Note::new(note_data),
    }
  }

  pub fn ui(&mut self, ui: &mut Ui) {
    self.note.ui(ui);
  }
}
