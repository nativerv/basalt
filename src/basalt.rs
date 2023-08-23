use std::{cell::RefCell, rc::Rc};

use crate::features::{note_graph::NoteGraphUi, note_preview::{NotePreviewUi, NoteData}};
use eframe::egui;

/// Global Basalt state
pub struct BasaltApp {
  note_graph_ui: NoteGraphUi,
  note_preview_ui: NotePreviewUi,
}

impl BasaltApp {
    pub fn new() -> Self {
      let my_str = include_str!("../tests/notes/test_markdown_first.md");
        Self {
            note_graph_ui: NoteGraphUi::default(),
            note_preview_ui: NotePreviewUi::new(Rc::new(RefCell::new(NoteData::new(my_str.to_string()) )) ),
        }
    }
}

impl eframe::App for BasaltApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.columns(2, |columns| {
        self.note_graph_ui.ui(&mut columns[0]);

        self.note_preview_ui.ui(&mut columns[1]);
      });
    });
  }
}
