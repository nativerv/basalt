#![allow(clippy::suboptimal_flops)]

use egui::{Color32, Stroke};
use std::marker::PhantomData;

use super::note_graph_ui::{NoteEdgeData, NoteNodeData};
use crate::lib::graph::{EdgeIncidents, Graph};

#[derive(PartialEq, Eq)]
pub struct MockGraph;

#[derive(PartialOrd, PartialEq, Eq, Hash, Clone, Copy)]
pub struct NodeId(usize);

#[derive(PartialEq, Clone, Copy)]
pub struct NodeData {
  radius: f32,
  fill: Color32,
  stroke: Stroke,
}
impl NoteNodeData for NodeData {
  fn fill(&self) -> Color32 {
    self.fill
  }
  fn stroke(&self) -> Stroke {
    self.stroke
  }
  fn radius(&self) -> f32 {
    self.radius
  }
}

pub struct NodeIter<'a> {
  index: usize,
  _marker: &'a PhantomData<()>,
}
pub struct NodeIterMut<'a> {
  _marker: &'a PhantomData<()>,
}

#[derive(PartialOrd, PartialEq, Eq, Hash, Clone, Copy)]
pub struct EdgeId(pub NodeId, pub NodeId);
#[derive(PartialEq, Clone, Copy)]
pub struct EdgeData {
  stroke: Stroke,
}
impl NoteEdgeData for EdgeData {
  fn stroke(&self) -> Stroke {
    self.stroke
  }
}

pub struct EdgeIter<'a> {
  index: usize,
  _marker: &'a PhantomData<()>,
}
pub struct EdgeIterMut<'a> {
  _marker: &'a PhantomData<()>,
}

#[rustfmt::skip]
pub const NODES: &[(NodeId, &NodeData)] = &[
  (NodeId(1), &NodeData { radius: 30.0, fill: Color32::WHITE, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(2), &NodeData { radius: 30.0, fill: Color32::WHITE, stroke: Stroke { width: 2.0, color: Color32::RED } }),
  (NodeId(3), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(4), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(5), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(6), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(7), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(8), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(9), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(10), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(11), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(12), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(13), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(14), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(15), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(16), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(17), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(18), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (NodeId(19), &NodeData { radius: 20.0, fill: Color32::GREEN, stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
];

#[rustfmt::skip]
const EDGES: &[(EdgeId, &EdgeData)] = &[
  (EdgeId(NodeId(1), NodeId(2)), &EdgeData { stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (EdgeId(NodeId(2), NodeId(1)), &EdgeData { stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (EdgeId(NodeId(3), NodeId(1)), &EdgeData { stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (EdgeId(NodeId(5), NodeId(1)), &EdgeData { stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (EdgeId(NodeId(6), NodeId(1)), &EdgeData { stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (EdgeId(NodeId(8), NodeId(1)), &EdgeData { stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (EdgeId(NodeId(9), NodeId(1)), &EdgeData { stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (EdgeId(NodeId(10), NodeId(1)), &EdgeData { stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (EdgeId(NodeId(11), NodeId(1)), &EdgeData { stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (EdgeId(NodeId(12), NodeId(1)), &EdgeData { stroke: Stroke { width: 2.0, color: Color32::WHITE } }),
  (EdgeId(NodeId(3), NodeId(9)), &EdgeData { stroke: Stroke { width: 2.0, color: Color32::RED } }),
  (EdgeId(NodeId(7), NodeId(6)), &EdgeData { stroke: Stroke { width: 2.0, color: Color32::RED } }),
  (EdgeId(NodeId(7), NodeId(3)), &EdgeData { stroke: Stroke { width: 2.0, color: Color32::RED } }),
  (EdgeId(NodeId(1), NodeId(5)), &EdgeData { stroke: Stroke { width: 2.0, color: Color32::RED } }),
  (EdgeId(NodeId(4), NodeId(2)), &EdgeData { stroke: Stroke { width: 2.0, color: Color32::RED } }),
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

  /// Iterate edges incident to the node
  fn iter_incidents(&self, _node_id: Self::NodeId) -> Self::EdgeIter {
    unimplemented!()
  }
  /// Iterate edges incident to the node (mutably)
  fn iter_incidents_mut(&mut self, _node_id: Self::NodeId) -> Self::EdgeIterMut {
    unimplemented!()
  }

  fn get_node(&self, id: Self::NodeId) -> &'a Self::NodeData {
    NODES
      .iter()
      .find(|node| node.0 == id)
      .map(|(_, node)| node)
      .unwrap()
  }
  fn get_node_mut(&mut self, _id: Self::NodeId) -> &'a mut Self::NodeData {
    unimplemented!()
  }

  fn get_edge(&self, id: Self::EdgeId) -> &'a Self::EdgeData {
    EDGES
      .iter()
      .find(|edge| edge.0 == id)
      .map(|(_, edge)| edge)
      .unwrap()
  }
  fn get_edge_mut(&mut self, _id: Self::EdgeId) -> &'a mut Self::EdgeData {
    unimplemented!()
  }
  /// Get incident nodes of the edge
  fn get_edge_incidents(&self, edge_id: Self::EdgeId) -> EdgeIncidents<Self::NodeId> {
    EDGES
      .iter()
      .find(|edge| edge.0 == edge_id)
      .map(|(EdgeId(node1_id, node2_id), ..)| EdgeIncidents {
        node_from: *node1_id,
        node_to: *node2_id,
      })
      .unwrap()
  }

  /// Adds a node to the graph
  fn add_node(&mut self, _data: &Self::NodeData) -> Self::NodeId {
    unimplemented!()
  }

  /// Removes a node from the graph, returning it's `NodeData` if the node was previously in the graph
  fn remove_node(&mut self, _data: Self::NodeId) -> Option<Self::NodeData> {
    unimplemented!()
  }

  /// Adds an edge to the graph
  fn add_edge(
    &mut self,
    _node_from: Self::NodeId,
    _node_to: Self::NodeId,
    _data: &Self::EdgeData,
  ) -> Self::EdgeId {
    unimplemented!()
  }

  /// Removes an edge from the graph, returning it's `EdgeData` if the edge was previously in the graph
  fn remove_edge(&mut self, _data: Self::EdgeId) -> Option<Self::EdgeData> {
    unimplemented!()
  }
}
