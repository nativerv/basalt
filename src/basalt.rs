use eframe::egui;

/// Global Basalt state
pub struct BasaltApp {
  name: String,
  age: u32,
}

impl Default for BasaltApp {
  fn default() -> Self {
    Self {
      name: String::from("nrv"),
      age: 42,
    }
  }
}

impl eframe::App for BasaltApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    egui::CentralPanel::default().show(ctx, |ui| {
      ui.heading("Basalt");
      ui.horizontal(|ui| {
        let name_label = ui.label("Your name: ");
        ui.text_edit_singleline(&mut self.name)
          .labelled_by(name_label.id);
      });
      ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
      if ui.button("Click each year").clicked() {
        self.age += 1;
      }
      ui.label(format!("Hello '{}', age {}", self.name, self.age));
    });
  }
}
