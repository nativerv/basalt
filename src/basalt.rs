use eframe::egui;

use crate::features::note_graph::NoteGraph;

/// Global Basalt state
#[derive(Default)]
pub struct BasaltApp {
  note_graph: NoteGraph,
}

impl eframe::App for BasaltApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      self.note_graph.ui(ui);
    });
  }
}
