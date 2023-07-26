use egui::Ui;

/// Show a button to reset a value to its default.
pub fn reset_button<T: Default>(ui: &mut Ui, value: &mut T) {
  reset_button_with(ui, value, T::default());
}

/// Show a button to reset a value to specified value.
pub fn reset_button_with<T>(ui: &mut Ui, value: &mut T, reset_value: T) {
  if ui.button("Reset").clicked() {
    *value = reset_value;
  }
}
