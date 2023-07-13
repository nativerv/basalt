use egui::{containers::*, widgets::*, *};
use epaint::CircleShape;
use std::f32::consts::TAU;
use std::collections::HashMap;
use crate::lib::graph::Graph;
use crate::features::note_graph::note_graph_mock::{NoteGraph, *};

#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct NoteGraphUi {}

impl Default for NoteGraphUi {
  fn default() -> Self {
    Self {}
  }
}

impl NoteGraphUi {
  pub fn ui(&mut self, ui: &mut Ui) {
    let painter = Painter::new(
      ui.ctx().clone(),
      ui.layer_id(),
      ui.available_rect_before_wrap(),
    );
    self.paint(&painter);

    // Make sure we allocate what we used (everything)
    // TODO: figure out why exactly this is needed (or not)
    ui.expand_to_include_rect(painter.clip_rect());

    Window::new("Options")
      .frame(Frame::popup(ui.style()))
      .show(&ui.ctx().clone(), |ui| {
        ui.set_max_width(512.0);
        ui.label("eblan");
        self.options_ui(ui);
      });
  }

  fn options_ui(&mut self, ui: &mut Ui) {
    ui.label("Eblan");
    egui::reset_button(ui, self);
  }

  fn paint(&mut self, painter: &Painter) {
    let mut shapes: Vec<Shape> = Vec::new();

    let graph = NoteGraph;

    const STEP: f32 = TAU / NODES.len() as f32;
    const RADIUS: f32 = 300.0;

    let node_positions = graph
      .iter_nodes()
      .enumerate()
      .map(|(index, (id, node))| {
        let x = painter.clip_rect().width() / 2.0 + RADIUS * (index as f32 * STEP).cos();
        let y = painter.clip_rect().height() / 2.0 + RADIUS * (index as f32 * STEP).sin();
        (id, (x, y))
      })
      .collect::<HashMap<_, _>>();

    for (id, node) in graph.iter_nodes() {
      let (x, y) = node_positions.get(&id).unwrap();
      shapes.push(Shape::Circle(CircleShape {
        center: pos2(*x, *y),
        radius: node.radius,
        fill: node.fill,
        stroke: node.stroke,
      }));
    }
    for (EdgeId(id_node1, id_node2), edge) in graph.iter_edges() {
      let (x1, y1) = node_positions.get(&id_node1).unwrap();
      let (x2, y2) = node_positions.get(&id_node2).unwrap();
      // let node1 = vec2(*x1, *y1);
      // let node2 = vec2(*x2, *y2);
      // let to_node1 = (node1 - node2).normalized();
      shapes.push(Shape::line_segment(
        [pos2(*x1, *y1), pos2(*x2, *y2)],
        edge.stroke,
      ));
    }
    painter.extend(shapes);
  }
}
