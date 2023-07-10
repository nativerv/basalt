use eframe::egui;

use crate::fractal_clock::{seconds_since_midnight, FractalClock};

/// Global Basalt state
#[derive(Default)]
pub struct BasaltApp {
  fractal_clock: FractalClock,
}

impl eframe::App for BasaltApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      self.fractal_clock.ui(ui, Some(seconds_since_midnight()));
    });
  }
}
