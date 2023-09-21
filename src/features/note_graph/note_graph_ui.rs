use crate::features::note_graph::note_graph_mock::{MockGraph, NodeId, NODES};
use crate::features::veins::Vein;
use crate::lib::fdp::eades_custom;
use crate::lib::graph::{EdgeIncidents, Graph};
use egui::{containers::*, *};
use epaint::CircleShape;
use std::cell::RefCell;
use std::collections::HashMap;
use std::f32::consts::TAU;
use std::rc::Rc;

/// NoteGraph ui state
pub struct NoteGraphUi {
  vein: Rc<RefCell<Vein>>,
  node_positions: eades_custom::NodePositions<NodeId>,
  note_graph: MockGraph,
  width: f32,
  height: f32,
  dragged_node: Option<NodeDrag>,
}

/// Stuff that NodeData of the graph has to have
pub trait NoteNodeData {
  fn fill(&self) -> Color32;
  fn stroke(&self) -> Stroke;
  fn radius(&self) -> f32;
}

/// Stuff that EdgeData of the graph has to have
pub trait NoteEdgeData {
  fn stroke(&self) -> Stroke;
}

/// Represents currently dragged node
struct NodeDrag {
  offset: Vec2,
  node_id: NodeId,
}

impl NoteGraphUi {
  pub fn new(vein: Rc<RefCell<Vein>>) -> Self {
    let note_graph = MockGraph;

    // Initial node placement: a circle
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
      vein,
      node_positions,
      note_graph,
      width: Default::default(),
      height: Default::default(),
      dragged_node: None,
    }
  }

  fn sync_node_positions(&mut self) {
    // Remove orphan nodes
    self.node_positions.retain(|&node_id, _| {
      let result = std::panic::catch_unwind(|| {
        self.note_graph.get_node(node_id)
      });

      if !result.is_ok() {
        log::debug!("retaining node: {node_id:?}");
      }
      result.is_ok()
    });

    fn cycle(n: f32, max: f32) -> f32 {
        let range = max * 2.0 + 1.0;
        ((n % range) + range) % range - max
    }

    // Add missing nodes
    let delta = 5.00;
    let modulo = 30.00;
    let mut current_x = 0.0;
    let mut current_y = 0.0 + delta;
    for (node_id, _) in self.note_graph.iter_nodes() {
      if let None = self.node_positions.get(&node_id) {
        self.node_positions.entry(node_id).or_insert(eades_custom::NodeFdpData {
          pos: vec2(current_x, current_y),
          force: vec2(current_x, current_y),
        });
        current_x = cycle(current_x + delta, modulo);
        current_y = cycle(current_y - delta, modulo);
      }
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

    // On click - find the node and put it as dragged one
    if ui.input(|input| input.pointer.primary_pressed()) {
      let pointer_pos = ui.input(|input| input.pointer.interact_pos());
      self.dragged_node = self.node_positions.iter().find_map(|(&node_id, node_fdp)| {
        let pointer_pos = pointer_pos.expect("pointer_pos is checked to not be `None` already");
        let pointer_to_node = node_fdp.pos - pointer_pos.to_vec2();
        let center = vec2(self.width / 2.0, self.height / 2.0);
        let node_radius = self.note_graph.get_node(node_id).radius();
        ((pointer_to_node + center).length() <= node_radius).then_some(NodeDrag {
          offset: pointer_to_node,
          node_id,
        })
      });
    }

    // When there is dragged node & we're dragging - drag it
    if ui.input(|input| input.pointer.primary_down()) && self.dragged_node.is_some() {
      let interact_pos = ui.input(|input| (input.pointer.interact_pos())).unwrap();
      self
        .node_positions
        .get_mut(&self.dragged_node.as_ref().unwrap().node_id)
        .unwrap()
        .pos = interact_pos.to_vec2() + self.dragged_node.as_ref().unwrap().offset;
    } else {
      // No longer dragged - relaase
      self.dragged_node = None;
    }

    // Popup window
    Window::new("Options")
      .frame(Frame::popup(ui.style()))
      .show(&ui.ctx().clone(), |ui| {
        ui.set_max_width(512.0);
        self.options_ui(ui);
      });
  }

  fn options_ui(&mut self, ui: &mut Ui) {
    if ui.button("Save").clicked() {
      self
        .vein
        .borrow()
        .write_config_value(&self.node_positions)
        .unwrap()
    }
    if ui.button("Load").clicked() {
      use std::{io::ErrorKind::*, error::Error};
      match Rc::clone(&self.vein).borrow().read_config_value() {
        Ok(node_positions) => {
          self.node_positions = node_positions;
          self.sync_node_positions();
        },
        Err(error) if error.kind() == InvalidData => log::error!("could not load node positions: corrupt node positions file: {error}"),
        Err(error) => log::error!("could not load node positions: unexpected error {error:#?}"),
      };
    }
    if ui.button("Step").clicked() {
      eades_custom::apply_forces(&self.note_graph, &mut self.node_positions);
      for (node_id, _) in self.note_graph.iter_nodes() {
        let eades_custom::NodeFdpData { pos, force } = self.node_positions.get_mut(&node_id).unwrap();
        *pos += *force;
      }
    }
    // NOTE: this bullshit seems to be correct way of doing that. Acquire a clone, pass it to the
    // closure and have it cloned there when it's called.
    // Actually can just move cloning inside `NoteGraphUi::new` and change the signature to &Rc,
    // but that one is more descriptive of what's happening with the Rc
    let vein_clone = Rc::clone(&self.vein);
    crate::ui::reset_button_with(ui, self, || Self::new(Rc::clone(&vein_clone)));
  }

  fn paint(&mut self, painter: &Painter) {
    let mut shapes: Vec<Shape> = Vec::new();

    // Progress the FDP
    // TODO: maybe decouple FDP force application from rendering/painting
    eades_custom::apply_forces(&self.note_graph, &mut self.node_positions);

    // Render nodes & apply the FDP forces to positions
    for (node_id, node) in self.note_graph.iter_nodes() {
      let eades_custom::NodeFdpData { pos, force } = self.node_positions.get_mut(&node_id).unwrap();

      // Apply forces
      // TODO: maybe decouple FDP force application from rendering/painting
      *pos += *force;

      // Render
      let pos = pos.to_pos2() + vec2(self.width / 2.0, self.height / 2.0);
      shapes.push(Shape::Circle(CircleShape {
        center: pos,
        radius: node.radius(),
        fill: node.fill(),
        stroke: node.stroke(),
      }));
    }

    // Render edges
    for (edge_id, edge) in self.note_graph.iter_edges() {
      let EdgeIncidents { node_from, node_to } = self.note_graph.get_edge_incidents(edge_id);
      let eades_custom::NodeFdpData { pos: start, .. } =
        self.node_positions.get(&node_from).unwrap();
      let eades_custom::NodeFdpData { pos: end, .. } = self.node_positions.get(&node_to).unwrap();
      let start_node = self.note_graph.get_node(node_from);
      let end_node = self.note_graph.get_node(node_to);
      let start = start.to_pos2() + vec2(self.width / 2.0, self.height / 2.0);
      let end = end.to_pos2() + vec2(self.width / 2.0, self.height / 2.0);
      let start_offset = (end - start).normalized() * start_node.radius();
      let end_offset = (start - end).normalized() * end_node.radius();

      let start = start + start_offset;
      let end = end + end_offset;

      // Draw a line from node to node
      shapes.push(Shape::line_segment([start, end], edge.stroke()));

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
        Shape::line_segment([end, arrow_part], edge.stroke())
      }))
    }
    painter.extend(shapes);
  }
}
