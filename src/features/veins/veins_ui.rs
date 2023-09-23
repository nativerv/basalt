use crate::features::veins::{VeinId, Veins};

pub struct VeinSelectionUi<'a> {
  veins: &'a Veins,
  current_vein: &'a mut Option<VeinId>,
}

impl<'a> VeinSelectionUi<'a> {
  pub fn new(veins: &'a Veins, current_vein: &'a mut Option<VeinId>) -> Self {
    Self {
      veins,
      current_vein,
    }
  }
}

impl egui::Widget for VeinSelectionUi<'_> {
  fn ui(self, ui: &mut egui::Ui) -> egui::Response {
    ui.vertical(|ui| {
      for (vein_id, ..) in self.veins.iter() {
        let is_selected = self
          .current_vein
          .as_ref()
          .map(|self_vein_id| self_vein_id == vein_id)
          .unwrap_or(false);
        if ui.selectable_label(is_selected, &**vein_id).clicked() {
          *self.current_vein = Some(vein_id.clone());
        }
      }
    })
    .response
  }
}
