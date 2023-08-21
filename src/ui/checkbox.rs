use egui::{
  epaint, pos2, vec2, NumExt, Response, Sense, Shape, TextStyle, Ui, Vec2, Widget, WidgetInfo,
  WidgetText, WidgetType,
};

pub struct Checkbox {
  checked: bool,
  text: WidgetText,
  interactive: bool,
}

impl Checkbox {
  pub fn new(checked: bool, text: impl Into<WidgetText>) -> Self {
    Checkbox {
      checked,
      text: text.into(),
      interactive: true,
    }
  }

  pub fn without_text(checked: bool) -> Self {
    Self::new(checked, WidgetText::default())
  }

  pub fn interactive(mut self, interactive: bool) -> Self {
    self.interactive = interactive;
    self
  }
}

impl Widget for Checkbox {
  fn ui(self, ui: &mut Ui) -> Response {
    let Checkbox {
      checked,
      text,
      interactive,
    } = self;

    let spacing = &ui.spacing();
    let icon_width = spacing.icon_width;
    let icon_spacing = spacing.icon_spacing;

    let (text, mut desired_size) = if text.is_empty() {
      (None, vec2(icon_width, 0.0))
    } else {
      let total_extra = vec2(icon_width + icon_spacing, 0.0);

      let wrap_width = ui.available_width() - total_extra.x;
      let text = text.into_galley(ui, None, wrap_width, TextStyle::Button);

      let mut desired_size = total_extra + text.size();
      desired_size = desired_size.at_least(spacing.interact_size);

      (Some(text), desired_size)
    };

    desired_size = desired_size.at_least(Vec2::splat(spacing.interact_size.y));
    desired_size.y = desired_size.y.max(icon_width);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, Sense::click());

    if response.clicked() {
      response.mark_changed();
    }
    response.widget_info(|| {
      WidgetInfo::selected(
        WidgetType::Checkbox,
        checked,
        text.as_ref().map_or("", |x| x.text()),
      )
    });

    if ui.is_rect_visible(rect) {
      // let visuals = ui.style().interact_selectable(&response, *checked); // too colorful
      let visuals = if interactive {
        ui.style().interact(&response)
      } else {
        ui.style().noninteractive()
      };
      let (small_icon_rect, big_icon_rect) = ui.spacing().icon_rectangles(rect);
      ui.painter().add(epaint::RectShape {
        rect: big_icon_rect.expand(visuals.expansion),
        rounding: visuals.rounding,
        fill: visuals.bg_fill,
        stroke: visuals.bg_stroke,
      });

      if checked {
        // Check mark:
        ui.painter().add(Shape::line(
          vec![
            pos2(small_icon_rect.left(), small_icon_rect.center().y),
            pos2(small_icon_rect.center().x, small_icon_rect.bottom()),
            pos2(small_icon_rect.right(), small_icon_rect.top()),
          ],
          visuals.fg_stroke,
        ));
      }
      if let Some(text) = text {
        let text_pos = pos2(
          rect.min.x + icon_width + icon_spacing,
          rect.center().y - 0.5 * text.size().y,
        );
        text.paint_with_visuals(ui.painter(), text_pos, visuals);
      }
    }

    response
  }
}
