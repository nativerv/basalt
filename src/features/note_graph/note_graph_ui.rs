use crate::features::note_graph::note_graph_mock::{NoteGraph, *};
use crate::lib::graph::Graph;
use egui::{containers::*, *};
use epaint::CircleShape;
use std::collections::HashMap;
use std::f32::consts::TAU;
use std::ops::{Deref, DerefMut};

/// Force
#[derive(PartialEq)]
struct NodeFdpData {
  force: Vec2,
  pos: Vec2,
}

type NodePositionsHashMap = HashMap<NodeId, NodeFdpData>;
#[derive(PartialEq)]
struct NodePositions(NodePositionsHashMap);
impl Deref for NodePositions {
  type Target = NodePositionsHashMap;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl DerefMut for NodePositions {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

// const REPULSION: f32 = 2.0;
// const SPRING: f32 = 1.0;
// const IDEAL_LENGTH: f32 = 30.0;
// const EPSILON: f32 = 0.1;
// const NUM_STEPS: usize = 2;

// fn f_rep(u: Vec2, v: Vec2) -> Vec2 {
//   (REPULSION / ((v - u).length().powi(2) + 0.0001)) * (v - u).normalized()
// }
//
// fn f_spring(u: Vec2, v: Vec2) -> Vec2 {
//   SPRING * ((v - u).length() / IDEAL_LENGTH).log10() * (v - u).normalized()
// }
//
// fn f_attr(u: Vec2, v: Vec2) -> Vec2 {
//   f_spring(u, v) - f_rep(u, v)
// }
//
// fn cooling(t: f32) -> f32 {
//   t * 0.5
// }
//
// fn force_directed(
//   g: &impl for<'a> Graph<'a, NodeId = NodeId, EdgeId = EdgeId>,
//   p: &mut NodePositions,
//   epsilon: f32,
//   k: usize,
// ) {
//   let mut t = 1;
//   let mut max_force = EPSILON + 0.01;
//
//   while t < k && max_force > epsilon {
//     for (u, ..) in g.iter_nodes() {
//       let sum_repulsion: Vec2 = g
//         .iter_nodes()
//         .filter(|(v, ..)| *v != u)
//         .map(|(v, ..)| {
//           let f_rep = f_rep(*p.0.get(&u).unwrap(), *p.0.get(&v).unwrap());
//           dbg!(f_rep);
//           f_rep
//         })
//         .fold(vec2(0.0, 0.0), |total_force, current_force| total_force + current_force);
//       let sum_adjacent_attraction: Vec2 = g
//         .iter_edges()
//         .filter_map(|(EdgeId(v1, v2), ..)| {
//           if v1 == v2 {
//             None
//           } else if v1 == u {
//             Some(v1)
//           } else if v2 == u {
//             Some(v2)
//           } else {
//             None
//           }
//         })
//         .map(|v| {
//           let f_attr = f_attr(*p.0.get(&u).unwrap(), *p.0.get(&v).unwrap());
//           dbg!(f_attr);
//           f_attr
//         })
//         .fold(vec2(0.0, 0.0), |total_force, current_force| total_force + current_force);
//       let f_u: Vec2 = sum_repulsion + sum_adjacent_attraction;
//       max_force = max_force.max(f_u.length());
//       println!("current force = {f_u:?}");
//       p.0
//         .insert(u, *p.0.get(&u).unwrap() + f_u * cooling(t as f32));
//     }
//     t += 1;
//   }
// }

const GRAVITY_CONSTANT: f32 = 0.05;
const FORCE_CONSTANT: f32 = 500.0;
const IDEAL_LENGTH: f32 = 3.0;

fn apply_forces(
  graph: &impl for<'a> Graph<'a, NodeId = NodeId, EdgeId = EdgeId>,
  node_positions: &mut NodePositions,
) {
  // apply force towards center
  for (node_id, ..) in graph.iter_nodes() {
    node_positions.get_mut(&node_id).unwrap().force =
      node_positions.get_mut(&node_id).unwrap().pos * -1.0 * GRAVITY_CONSTANT;
  }

  // apply repulsive force between nodes
  for (index, (node_id1, ..)) in graph.iter_nodes().enumerate() {
    for (node_id2, ..) in graph.iter_nodes().skip(index + 1)
    {
      let node1_fdp = node_positions.get(&node_id1).unwrap();
      let node2_fdp = node_positions.get(&node_id2).unwrap();
      let direction = node2_fdp.pos - node1_fdp.pos;
      let force = (direction / (direction.length().powi(2))) * FORCE_CONSTANT;
      node_positions.get_mut(&node_id1).unwrap().force += -force;
      node_positions.get_mut(&node_id2).unwrap().force += force;
    }
  }

  // apply forces applied by connections (springs)
  for (EdgeId(node_id1, node_id2), ..) in graph.iter_edges() {
    let node1_fdp = node_positions.get(&node_id1).unwrap();
    let node2_fdp = node_positions.get(&node_id2).unwrap();
    let dis = (node1_fdp.pos - node2_fdp.pos) / 8.0;
    //let diff = dis.length() - IDEAL_LENGTH;
    //node_positions.get_mut(&node_id1).unwrap().force += -dis + vec2(-diff, -diff);
    //node_positions.get_mut(&node_id2).unwrap().force += dis + vec2(diff, diff);
    node_positions.get_mut(&node_id1).unwrap().force += -dis;
    node_positions.get_mut(&node_id2).unwrap().force += dis;
  }

  //for (node_id1, ..) in graph.iter_nodes() {
  //  println!(
  //    "current force = {force:?}",
  //    force = node_positions.get(&node_id1).unwrap().force
  //  );
  //}
}

#[derive(PartialEq)]
// #[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
// #[cfg_attr(feature = "serde", serde(default))]
pub struct NoteGraphUi {
  node_positions: NodePositions,
  note_graph: NoteGraph,
  width: f32,
  height: f32,
}

impl Default for NoteGraphUi {
  fn default() -> Self {
    // use rand::Rng;
    // let mut rng = rand::thread_rng();

    let note_graph = NoteGraph;
    const STEP: f32 = TAU / NODES.len() as f32;
    const RADIUS: f32 = 300.0;
    const _RNG_SCALE: f32 = 300.0;
    const OFFSET_X: f32 = 300.0;
    const OFFSET_Y: f32 = 300.0;
    let node_positions = NodePositions(
      note_graph
        .iter_nodes()
        .enumerate()
        .map(|(index, (id, _))| {
          let x = RADIUS * ((index as f32 * STEP).cos());
          let y = RADIUS * ((index as f32 * STEP).sin());
          (
            id,
            NodeFdpData {
              pos: vec2(x, y),
              force: Vec2::default(),
            },
          )
        })
        .collect::<HashMap<_, _>>(),
    );
    // let mut node_positions = note_graph
    //   .iter_nodes()
    //   .fold(NodePositions(HashMap::new()), |mut node_positions, (node_id, ..)| {
    //     node_positions.insert(
    //       node_id,
    //       vec2(OFFSET_X + rng.gen::<f32>() * RNG_SCALE, OFFSET_Y + rng.gen::<f32>() * RNG_SCALE)
    //     );
    //     node_positions
    //  });

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
      apply_forces(&self.note_graph, &mut self.node_positions);
      for (id, node) in self.note_graph.iter_nodes() {
        let NodeFdpData { pos, force } = self.node_positions.get_mut(&id).unwrap();
        *pos += *force;
      }
    }
    egui::reset_button(ui, self);
  }

  fn paint(&mut self, painter: &Painter) {
    let mut shapes: Vec<Shape> = Vec::new();

    // const STEP: f32 = TAU / NODES.len() as f32;
    // const RADIUS: f32 = 300.0;
    // let node_positions = graph
    //   .iter_nodes()
    //   .enumerate()
    //   .map(|(index, (id, node))| {
    //     let x = painter.clip_rect().width() / 2.0 + RADIUS * (index as f32 * STEP).cos();
    //     let y = painter.clip_rect().height() / 2.0 + RADIUS * (index as f32 * STEP).sin();
    //     (id, (x, y))
    //   })
    //   .collect::<HashMap<_, _>>();
    apply_forces(&self.note_graph, &mut self.node_positions);
    for (id, node) in self.note_graph.iter_nodes() {
      let NodeFdpData { pos, force } = self.node_positions.get_mut(&id).unwrap();
      *pos += *force;
      let mut pos = pos.to_pos2() + vec2(self.width / 2.0, self.height / 2.0);
      shapes.push(Shape::Circle(CircleShape {
        center: pos,
        radius: node.radius,
        fill: node.fill,
        stroke: node.stroke,
      }));
    }
    for (EdgeId(id_node1, id_node2), edge) in self.note_graph.iter_edges() {
      let NodeFdpData { pos: start, .. } = self.node_positions.get(&id_node1).unwrap();
      let NodeFdpData { pos: end, .. } = self.node_positions.get(&id_node2).unwrap();
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
