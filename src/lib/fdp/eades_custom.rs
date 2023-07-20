use crate::lib::graph::{EdgeIncidents, Graph};
use egui::Vec2;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

/// Force-directed placement data
pub struct NodeFdpData {
  pub force: Vec2,
  pub pos: Vec2,
}

pub type NodePositionsHashMap<NodeId> = HashMap<NodeId, NodeFdpData>;
pub struct NodePositions<NodeId: Hash + Eq>(pub NodePositionsHashMap<NodeId>);

impl<NodeId> Deref for NodePositions<NodeId>
where
  NodeId: Hash + Eq,
{
  type Target = NodePositionsHashMap<NodeId>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}
impl<NodeId> DerefMut for NodePositions<NodeId>
where
  NodeId: Hash + Eq,
{
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

pub const GRAVITY_CONSTANT: f32 = 0.1;
pub const FORCE_CONSTANT: f32 = 1000.0;
pub const IDEAL_LENGTH: f32 = 25.0;

pub fn apply_forces<NodeId, EdgeId>(
  graph: &impl for<'a> Graph<'a, NodeId = NodeId, EdgeId = EdgeId>,
  node_positions: &mut NodePositions<NodeId>,
) where
  NodeId: Hash + Eq,
{
  // apply force towards center
  for (node_id, ..) in graph.iter_nodes() {
    node_positions.get_mut(&node_id).unwrap().force =
      node_positions.get_mut(&node_id).unwrap().pos * -1.0 * GRAVITY_CONSTANT;
  }

  // apply repulsive force between nodes
  let f_rep = |direction: Vec2| (direction / (direction.length().powi(2))) * FORCE_CONSTANT;
  for (index, (node_id1, ..)) in graph.iter_nodes().enumerate() {
    for (node_id2, ..) in graph.iter_nodes().skip(index + 1) {
      let node1_fdp = node_positions.get(&node_id1).unwrap();
      let node2_fdp = node_positions.get(&node_id2).unwrap();
      let direction = node2_fdp.pos - node1_fdp.pos;
      let force = f_rep(direction);
      node_positions.get_mut(&node_id1).unwrap().force += -force;
      node_positions.get_mut(&node_id2).unwrap().force += force;
    }
  }

  // apply forces applied by connections (springs)
  for (edge_id, ..) in graph.iter_edges() {
    let EdgeIncidents {
      node_from: node1_id,
      node_to: node2_id,
    } = graph.get_edge_incidents(edge_id);
    let node1_fdp = node_positions.get(&node1_id).unwrap();
    let node2_fdp = node_positions.get(&node2_id).unwrap();
    // TODO: document magic number
    let dis = (node1_fdp.pos - node2_fdp.pos) / 8.0;
    let diff = (dis.length() / IDEAL_LENGTH).log10();
    node_positions.get_mut(&node1_id).unwrap().force += -dis * Vec2::splat(diff);
    node_positions.get_mut(&node2_id).unwrap().force += dis * Vec2::splat(diff);
  }
}
