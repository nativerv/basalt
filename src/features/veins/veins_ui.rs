use crate::{
  features::veins::{VeinId, Veins},
  ui::SelectableItem,
};

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
    use egui::{Color32, Pos2, Rect, Sense, Stroke};
    for (vein_id, maybe_vein) in self.veins.iter() {
      let is_selected = self
        .current_vein
        .as_ref()
        .map(|self_vein_id| self_vein_id == vein_id)
        .unwrap_or(false);
      ui.scope(|ui| {
        if maybe_vein.is_err() {
          ui.style_mut().visuals.selection.bg_fill = Color32::DARK_RED;
        }
        if is_selected {
          ui.style_mut().visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, Color32::WHITE);
        }
        //ui.set_max_width(200.);
        let selectable_label = SelectableItem::new(true, &**vein_id);
        if ui.add(selectable_label).clicked() {
          *self.current_vein = Some(vein_id.clone());
        }
      });
    }
    ui.allocate_rect(
      Rect {
        min: Pos2::ZERO,
        max: Pos2::ZERO,
      },
      Sense {
        click: false,
        drag: false,
        focusable: false,
      },
    )
  }
}
