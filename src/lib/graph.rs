/// Represents a genreic graph structure
/// It has to have an iterator over all nodes and all edges,
/// as well as getters for nodes and edges.
/// Iterator should be over pairs of nodes
pub trait Graph<'a> {
  type NodeId: 'a;
  type NodeData: 'a;

  type EdgeId: 'a;
  type EdgeData: 'a;

  type NodeIter: Iterator<Item = (Self::NodeId, &'a Self::NodeData)> + 'a;
  type NodeIterMut: Iterator<Item = (Self::NodeId, &'a mut Self::NodeData)> + 'a;

  type EdgeIter: Iterator<Item = (Self::EdgeId, &'a Self::EdgeData)> + 'a;
  type EdgeIterMut: Iterator<Item = (Self::EdgeId, &'a mut Self::EdgeData)> + 'a;

  fn iter_nodes(&self) -> Self::NodeIter;
  fn iter_nodes_mut(&mut self) -> Self::NodeIterMut;

  fn iter_edges(&self) -> Self::EdgeIter;
  fn iter_edges_mut(&mut self) -> Self::EdgeIterMut;

  fn get_node(&self, id: Self::NodeId) -> &'a Self::NodeData;
  fn get_node_mut(&mut self, id: Self::NodeId) -> &'a mut Self::NodeData;

  fn get_edge(&self, id: Self::EdgeId) -> &'a Self::EdgeData;
  fn get_edge_mut(&mut self, id: Self::EdgeId) -> &'a mut Self::EdgeData;
}
