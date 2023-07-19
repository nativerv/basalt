use crate::features::note_graph::note_graph_mock::{NoteGraph, *};
use crate::lib::fdp::eades_custom;
use crate::lib::graph::Graph;
use egui::{containers::*, *};
use epaint::CircleShape;
use std::collections::HashMap;
use std::f32::consts::TAU;

// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "serde", serde(default))]
pub struct NoteGraphUi {
  node_positions: eades_custom::NodePositions<NodeId>,
  note_graph: NoteGraph,
  width: f32,
  height: f32,
}

impl Default for NoteGraphUi {
  fn default() -> Self {
    let note_graph = NoteGraph;
    const STEP: f32 = TAU / NODES.len() as f32;
    const RADIUS: f32 = 300.0;
    let node_positions = eades_custom::NodePositions(
      note_graph
        .iter_nodes()
        .enumerate()
        .map(|(index, (id, _))| {
          let x = RADIUS * ((index as f32 * STEP).cos());
          let y = RADIUS * ((index as f32 * STEP).sin());
          (
            id,
            eades_custom::NodeFdpData {
              pos: vec2(x, y),
              force: Vec2::default(),
            },
          )
        })
        .collect::<HashMap<_, _>>(),
    );

    Self {
      node_positions,
      note_graph,
      width: 0.0,
      height: 0.0,
    }
  }
}

impl NoteGraphUi {
  pub fn ui(&mut self, ui: &mut Ui) {
    let painter = Painter::new(
      ui.ctx().clone(),
      ui.layer_id(),
      ui.available_rect_before_wrap(),
    );
    self.width = painter.clip_rect().width();
    self.height = painter.clip_rect().height();
    self.paint(&painter);

    // Required, or else will redraw only on mouse movement/other interactions
    ui.ctx().request_repaint();

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
    if ui.button("Step").clicked() {
      eades_custom::apply_forces(&self.note_graph, &mut self.node_positions);
      for (id, _node) in self.note_graph.iter_nodes() {
        let eades_custom::NodeFdpData { pos, force } = self.node_positions.get_mut(&id).unwrap();
        *pos += *force;
      }
    }
    crate::ui::reset_button(ui, self);
  }

  fn paint(&mut self, painter: &Painter) {
    let mut shapes: Vec<Shape> = Vec::new();

    // Progress the FDP
    eades_custom::apply_forces(&self.note_graph, &mut self.node_positions);

    // Render nodes & apply the FDP forces to positions
    for (id, node) in self.note_graph.iter_nodes() {
      let eades_custom::NodeFdpData { pos, force } = self.node_positions.get_mut(&id).unwrap();

      // Apply forces
      *pos += *force;

      // Render
      let pos = pos.to_pos2() + vec2(self.width / 2.0, self.height / 2.0);
      shapes.push(Shape::Circle(CircleShape {
        center: pos,
        radius: node.radius,
        fill: node.fill,
        stroke: node.stroke,
      }));
    }

    // Render edges
    for (EdgeId(id_node1, id_node2), edge) in self.note_graph.iter_edges() {
      let eades_custom::NodeFdpData { pos: start, .. } =
        self.node_positions.get(&id_node1).unwrap();
      let eades_custom::NodeFdpData { pos: end, .. } = self.node_positions.get(&id_node2).unwrap();
      let start = start.to_pos2() + vec2(self.width / 2.0, self.height / 2.0);
      let end = end.to_pos2() + vec2(self.width / 2.0, self.height / 2.0);

      // Draw a line from node to node
      shapes.push(Shape::line_segment([start, end], edge.stroke));

      // Draw arrow head
      const ARROW_HEAD_ANGLE_DEGREES: f32 = 20.0;
      const ARROW_HEAD_LENGTH: f32 = 13.0;
      let theta = TAU / 360.0 * ARROW_HEAD_ANGLE_DEGREES;
      let norm_towards_start = (start - end).normalized() * ARROW_HEAD_LENGTH;
      shapes.extend([theta, -theta].into_iter().map(|theta| {
        let arrow_part = pos2(
          norm_towards_start
            .x
            .mul_add(theta.cos(), -norm_towards_start.y * theta.sin()),
          norm_towards_start
            .x
            .mul_add(theta.sin(), norm_towards_start.y * theta.cos()),
        ) + end.to_vec2();
        Shape::line_segment([end, arrow_part], edge.stroke)
      }))
    }
    painter.extend(shapes);
  }
}
