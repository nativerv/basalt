use eframe::egui;
use crate::lib::images_cache::ImagesCache;
use crate::features::{note_graph::NoteGraphUi, note_preview::NotePreviewUi};

/// Global Basalt state
#[derive(Default)]
pub struct BasaltApp {
  note_graph_ui: NoteGraphUi,
  note_preview_ui: NotePreviewUi,
}

impl eframe::App for BasaltApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      self.note_preview_ui.ui(ui);

    });
  }
}
