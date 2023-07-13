#![allow(clippy::suboptimal_flops)]

use egui::{containers::*, widgets::*, *};
use epaint::CircleShape;
use std::collections::HashMap;
use std::f32::consts::TAU;
use std::marker::PhantomData;

/// Time of day as seconds since midnight. Used for clock in demo app.
pub fn seconds_since_midnight() -> f64 {
  use chrono::Timelike;
  let time = chrono::Local::now().time();
  time.num_seconds_from_midnight() as f64 + 1e-9 * (time.nanosecond() as f64)
}

#[derive(PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(default))]
pub struct NoteGraph {}

impl Default for NoteGraph {
  fn default() -> Self {
    Self {}
  }
}

impl NoteGraph {
  pub fn ui(&mut self, ui: &mut Ui) {
    //ui.ctx().request_repaint();
    let painter = Painter::new(
      ui.ctx().clone(),
      ui.layer_id(),
      ui.available_rect_before_wrap(),
    );
    self.paint(&painter);
    // Make sure we allocate what we used (everything)
    // ui.expand_to_include_rect(painter.clip_rect());

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

    // ui.checkbox(&mut self.paused, "Paused");
    // ui.add(Slider::new(&mut self.zoom, 0.0..=1.0).text("zoom"));
    // ui.add(Slider::new(&mut self.start_line_width, 0.0..=5.0).text("Start line width"));
    // ui.add(Slider::new(&mut self.depth, 0..=14).text("depth"));
    // ui.add(Slider::new(&mut self.length_factor, 0.0..=1.0).text("length factor"));
    // ui.add(Slider::new(&mut self.luminance_factor, 0.0..=1.0).text("luminance factor"));
    // ui.add(Slider::new(&mut self.width_factor, 0.0..=1.0).text("width factor"));
    // ui.add(Slider::new(&mut self.offset, 0..=60).text("offset"));

    egui::reset_button(ui, self);
  }

  fn paint(&mut self, painter: &Painter) {
    // struct Hand {
    //   length: f32,
    //   angle: f32,
    //   vec: Vec2,
    // }
    //
    // impl Hand {
    //   fn from_length_angle(length: f32, angle: f32) -> Self {
    //     Self {
    //       length,
    //       angle,
    //       vec: length * Vec2::angled(angle),
    //     }
    //   }
    // }
    //
    // let angle_from_period =
    //   |period| TAU * (self.time.rem_euclid(period) / period) as f32 + -TAU / 4.0;
    //
    // let hands = [
    //   // Second hand:
    //   Hand::from_length_angle(self.length_factor, angle_from_period(60.0)),
    //   // Minute hand:
    //   Hand::from_length_angle(self.length_factor, angle_from_period(60.0 * 60.0)),
    //   // Hour hand:
    //   Hand::from_length_angle(0.5, angle_from_period(12.0 * 60.0 * 60.0)),
    // ];
    //
    // let mut shapes: Vec<Shape> = Vec::new();
    //
    // let rect = painter.clip_rect();
    // let to_screen = emath::RectTransform::from_to(
    //   Rect::from_center_size(Pos2::ZERO, rect.square_proportions() / self.zoom),
    //   rect,
    // );
    //
    // let mut paint_line = |points: [Pos2; 2], color: Color32, width: f32| {
    //   let line = [to_screen * points[0], to_screen * points[1]];
    //
    //   // culling
    //   // if rect.intersects(Rect::from_two_pos(line[0], line[1])) {
    //     shapes.push(Shape::line_segment(line, (width, color)));
    //   // }
    // };
    //
    // let hand_rotations = [
    //   hands[0].angle - hands[2].angle + TAU / 2.0,
    //   hands[1].angle - hands[2].angle + TAU / 2.0,
    // ];
    //
    // let hand_rotors = [
    //   hands[0].length * emath::Rot2::from_angle(hand_rotations[0]),
    //   hands[1].length * emath::Rot2::from_angle(hand_rotations[1]),
    // ];
    //
    // #[derive(Clone, Copy)]
    // struct Node {
    //   pos: Pos2,
    //   dir: Vec2,
    // }
    //
    // let mut nodes = Vec::new();
    //
    // let mut width = self.start_line_width;
    //
    // for (i, hand) in hands.iter().enumerate() {
    //   let center = pos2(0.0, 0.0);
    //   let end = center + hand.vec;
    //   paint_line([center, end], Color32::from_additive_luminance(255), width);
    //   if i < 2 {
    //     nodes.push(Node {
    //       pos: end,
    //       dir: hand.vec,
    //     });
    //   }
    // }
    //
    // let mut luminance = 0.7; // Start dimmer than main hands
    //
    // let mut new_nodes = Vec::new();
    // for _ in 0..self.depth {
    //   new_nodes.clear();
    //   new_nodes.reserve(nodes.len() * 2);
    //
    //   luminance *= self.luminance_factor;
    //   width *= self.width_factor;
    //
    //   let luminance_u8 = (255.0 * luminance).round() as u8;
    //   if luminance_u8 == 0 {
    //     break;
    //   }
    //
    //   for &rotor in &hand_rotors {
    //     for a in &nodes {
    //       let new_dir = rotor * a.dir;
    //       let b = Node {
    //         pos: a.pos + new_dir,
    //         dir: new_dir,
    //       };
    //       paint_line(
    //         [a.pos, b.pos],
    //         Color32::from_additive_luminance(luminance_u8),
    //         width,
    //       );
    //       new_nodes.push(b);
    //     }
    //   }
    //
    //   std::mem::swap(&mut nodes, &mut new_nodes);
    // }
    // self.line_count = shapes.len();

    let mut shapes: Vec<Shape> = Vec::new();

    let graph = MockGraph;

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
      shapes.push(Shape::line_segment(
        [pos2(*x1, *y1), pos2(*x2, *y2)],
        edge.stroke,
      ));
    }
    //shapes.push(Shape::cicrle([pos2(0.0, 0.0), pos2(300.0, 300.0)], (2.0, Color32::from_additive_luminance(255))));
    painter.extend(shapes);
  }
}

use crate::lib::graph::Graph;
struct MockGraph;

#[derive(PartialOrd, PartialEq, Eq, Hash, Clone, Copy)]
struct NodeId(usize);
#[derive(PartialEq, Clone, Copy)]
struct NodeData {
  radius: f32,
  fill: Color32,
  stroke: Stroke,
}

struct NodeIter<'a> {
  index: usize,
  _marker: &'a PhantomData<()>,
}
struct NodeIterMut<'a> {
  _marker: &'a PhantomData<()>,
}

#[derive(PartialOrd, PartialEq, Eq, Hash, Clone, Copy)]
struct EdgeId(NodeId, NodeId);
#[derive(PartialEq, Clone, Copy)]
struct EdgeData {
  stroke: Stroke,
}
struct EdgeIter<'a> {
  index: usize,
  _marker: &'a PhantomData<()>,
}
struct EdgeIterMut<'a> {
  _marker: &'a PhantomData<()>,
}

const NODES: &[(NodeId, &NodeData)] = &[
  (
    NodeId(1),
    &NodeData {
      radius: 30.0,
      fill: Color32::WHITE,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    NodeId(2),
    &NodeData {
      radius: 30.0,
      fill: Color32::WHITE,
      stroke: Stroke {
        width: 2.0,
        color: Color32::RED,
      },
    },
  ),
  (
    NodeId(3),
    &NodeData {
      radius: 20.0,
      fill: Color32::GREEN,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    NodeId(4),
    &NodeData {
      radius: 20.0,
      fill: Color32::GREEN,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    NodeId(5),
    &NodeData {
      radius: 20.0,
      fill: Color32::GREEN,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    NodeId(6),
    &NodeData {
      radius: 20.0,
      fill: Color32::GREEN,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    NodeId(7),
    &NodeData {
      radius: 20.0,
      fill: Color32::GREEN,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    NodeId(8),
    &NodeData {
      radius: 20.0,
      fill: Color32::GREEN,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    NodeId(9),
    &NodeData {
      radius: 20.0,
      fill: Color32::GREEN,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    NodeId(10),
    &NodeData {
      radius: 20.0,
      fill: Color32::GREEN,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    NodeId(11),
    &NodeData {
      radius: 20.0,
      fill: Color32::GREEN,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    NodeId(12),
    &NodeData {
      radius: 20.0,
      fill: Color32::GREEN,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    NodeId(13),
    &NodeData {
      radius: 20.0,
      fill: Color32::GREEN,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    NodeId(14),
    &NodeData {
      radius: 20.0,
      fill: Color32::GREEN,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    NodeId(15),
    &NodeData {
      radius: 20.0,
      fill: Color32::GREEN,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    NodeId(16),
    &NodeData {
      radius: 20.0,
      fill: Color32::GREEN,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    NodeId(17),
    &NodeData {
      radius: 20.0,
      fill: Color32::GREEN,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    NodeId(18),
    &NodeData {
      radius: 20.0,
      fill: Color32::GREEN,
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
];

const EDGES: &[(EdgeId, &EdgeData)] = &[
  (
    EdgeId(NodeId(1), NodeId(2)),
    &EdgeData {
      stroke: Stroke {
        width: 2.0,
        color: Color32::WHITE,
      },
    },
  ),
  (
    EdgeId(NodeId(2), NodeId(3)),
    &EdgeData {
      stroke: Stroke {
        width: 2.0,
        color: Color32::RED,
      },
    },
  ),
  (
    EdgeId(NodeId(5), NodeId(6)),
    &EdgeData {
      stroke: Stroke {
        width: 2.0,
        color: Color32::RED,
      },
    },
  ),
  (
    EdgeId(NodeId(3), NodeId(9)),
    &EdgeData {
      stroke: Stroke {
        width: 2.0,
        color: Color32::RED,
      },
    },
  ),
  (
    EdgeId(NodeId(7), NodeId(6)),
    &EdgeData {
      stroke: Stroke {
        width: 2.0,
        color: Color32::RED,
      },
    },
  ),
  (
    EdgeId(NodeId(7), NodeId(3)),
    &EdgeData {
      stroke: Stroke {
        width: 2.0,
        color: Color32::RED,
      },
    },
  ),
  (
    EdgeId(NodeId(1), NodeId(5)),
    &EdgeData {
      stroke: Stroke {
        width: 2.0,
        color: Color32::RED,
      },
    },
  ),
  (
    EdgeId(NodeId(4), NodeId(2)),
    &EdgeData {
      stroke: Stroke {
        width: 2.0,
        color: Color32::RED,
      },
    },
  ),
];

const NODES_ITER: NodeIter = NodeIter {
  index: 0,
  _marker: &PhantomData,
};
const EDGES_ITER: EdgeIter = EdgeIter {
  index: 0,
  _marker: &PhantomData,
};

impl<'a> Iterator for NodeIter<'a> {
  type Item = (NodeId, &'a NodeData);
  fn next(&mut self) -> Option<Self::Item> {
    let current = NODES.get(self.index);
    self.index += 1;
    current.copied()
  }
}

impl<'a> Iterator for NodeIterMut<'a> {
  type Item = (NodeId, &'a mut NodeData);
  fn next(&mut self) -> Option<Self::Item> {
    unimplemented!()
  }
}

impl<'a> Iterator for EdgeIter<'a> {
  type Item = (EdgeId, &'a EdgeData);
  fn next(&mut self) -> Option<Self::Item> {
    let current = EDGES.get(self.index);
    self.index += 1;
    current.copied()
  }
}

impl<'a> Iterator for EdgeIterMut<'a> {
  type Item = (EdgeId, &'a mut EdgeData);
  fn next(&mut self) -> Option<Self::Item> {
    unimplemented!()
  }
}

impl<'a> Graph<'a> for MockGraph {
  type NodeId = NodeId;
  type NodeData = NodeData;

  type EdgeId = EdgeId;
  type EdgeData = EdgeData;

  type NodeIter = NodeIter<'a>;
  type NodeIterMut = NodeIterMut<'a>;

  type EdgeIter = EdgeIter<'a>;
  type EdgeIterMut = EdgeIterMut<'a>;

  fn iter_nodes(&self) -> Self::NodeIter {
    NODES_ITER
  }
  fn iter_nodes_mut(&mut self) -> Self::NodeIterMut {
    unimplemented!()
  }

  fn iter_edges(&self) -> Self::EdgeIter {
    EDGES_ITER
  }
  fn iter_edges_mut(&mut self) -> Self::EdgeIterMut {
    unimplemented!()
  }

  fn get_node(&self, id: Self::NodeId) -> &'a Self::NodeData {
    NODES
      .iter()
      .find(|node| node.0 == id)
      .map(|(_, node)| node)
      .unwrap()
  }
  fn get_node_mut(&mut self, id: Self::NodeId) -> &'a mut Self::NodeData {
    unimplemented!()
  }

  fn get_edge(&self, id: Self::EdgeId) -> &'a Self::EdgeData {
    EDGES
      .iter()
      .find(|edge| edge.0 == id)
      .map(|(_, edge)| edge)
      .unwrap()
  }
  fn get_edge_mut(&mut self, id: Self::EdgeId) -> &'a mut Self::EdgeData {
    unimplemented!()
  }
}
