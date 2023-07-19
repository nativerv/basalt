pub struct EdgeIncidents<T> {
  pub node_from: T,
  pub node_to: T,
}

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

  /// Iterate edges incident to the node
  fn iter_incidents(&self, node_id: Self::NodeId) -> Self::EdgeIter;
  /// Iterate edges incident to the node (mutably)
  fn iter_incidents_mut(&mut self, node_id: Self::NodeId) -> Self::EdgeIterMut;

  fn get_node(&self, id: Self::NodeId) -> &'a Self::NodeData;
  fn get_node_mut(&mut self, id: Self::NodeId) -> &'a mut Self::NodeData;

  fn get_edge(&self, id: Self::EdgeId) -> &'a Self::EdgeData;
  fn get_edge_mut(&mut self, id: Self::EdgeId) -> &'a mut Self::EdgeData;

  /// Get incident nodes of the edge
  fn get_edge_incidents(&self, edge_id: Self::EdgeId) -> EdgeIncidents<(Self::NodeId, &'a Self::NodeData)>;

  /// Get incident nodes of the edge (mutably)
  fn get_edge_incidents_mut(&mut self, edge_id: Self::EdgeId) -> EdgeIncidents<(Self::NodeId, &'a mut Self::NodeData)>;

  /// Adds a node to the graph
  fn add_node(&mut self, data: &Self::NodeData) -> Self::NodeId;

  /// Removes a node from the graph, returning it's `NodeData` if the node was previously in the graph
  fn remove_node(&mut self, data: Self::NodeId) -> Option<Self::NodeData>;

  /// Adds an edge to the graph
  fn add_edge(&mut self, node_from: Self::NodeId, node_to: Self::NodeId, data: &Self::EdgeData) -> Self::EdgeId;

  /// Removes an edge from the graph, returning it's `EdgeData` if the edge was previously in the graph
  fn remove_edge(&mut self, data: Self::EdgeId) -> Option<Self::EdgeData>;
}
