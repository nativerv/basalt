use std::collections::HashMap;
use pulldown_cmark::{Event, LinkType, Parser, Tag};
use super:: models::{Link, LinkEdge, LinkNode, LinkNodeId};

use crate::lib::graph::Graph;

#[derive(Debug, Clone)]
pub struct ANoteGraph<NodeId, EdgeData, NodeData> {
    pub nodes: HashMap<NodeId, NodeData>,
    pub adjacency: HashMap<NodeId, Vec<AAdjacement<NodeId, EdgeData>>>
}

impl<NodeId, E, N> Default for ANoteGraph<NodeId, E, N>
where
    NodeId: std::hash::Hash + Eq,
    N: ANode,
    E: AEdge,
{
    fn default() -> Self {
        ANoteGraph {
            nodes: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }
}

pub trait AEdge { }

pub trait ANode { }

#[derive(PartialOrd, PartialEq, Eq, Hash, Clone, Copy)]
pub struct ANodeId(usize);

#[derive(Debug, Clone)]
pub struct  AAdjacement<NodeId, Edge>(pub NodeId, pub Edge);
//
// impl<'a> Graph<'a> for ANoteGraph<LinkNodeId, LinkEdge, LinkNode> {
//     type NodeId = LinkNodeId;
//
//     type NodeData = LinkNode;
//
//     type EdgeId = ;
//
//     type EdgeData = LinkEdge;
//
//     type NodeIter = ;
//
//     type NodeIterMut = ;
//
//     type EdgeIter = ;
//
//     type EdgeIterMut = ;
//
//     fn iter_nodes(&self) -> Self::NodeIter {
//         todo!()
//     }
//
//     fn iter_nodes_mut(&mut self) -> Self::NodeIterMut {
//         todo!()
//     }
//
//     fn iter_edges(&self) -> Self::EdgeIter {
//         todo!()
//     }
//
//     fn iter_edges_mut(&mut self) -> Self::EdgeIterMut {
//         todo!()
//     }
//
//     fn iter_incidents(&self, node_id: Self::NodeId) -> Self::EdgeIter {
//         todo!()
//     }
//
//     fn iter_incidents_mut(&mut self, node_id: Self::NodeId) -> Self::EdgeIterMut {
//         todo!()
//     }
//
//     fn get_node(&self, id: Self::NodeId) -> &'a Self::NodeData {
//         self.nodes.get(&id)
//     }
//
//     fn get_node_mut(&mut self, id: Self::NodeId) -> &'a mut Self::NodeData {
//         todo!()
//     }
//
//     fn get_edge(&self, id: Self::EdgeId) -> &'a Self::EdgeData {
//       // self.adjacency
//         todo!()
//     }
//
//     fn get_edge_mut(&mut self, id: Self::EdgeId) -> &'a mut Self::EdgeData {
//         todo!()
//     }
//
//     fn get_edge_incidents(&self, edge_id: Self::EdgeId) -> crate::lib::graph::EdgeIncidents<Self::NodeId> {
//         todo!()
//     }
//
//     fn add_node(&mut self, data: &Self::NodeData) -> Self::NodeId {
//         todo!()
//     }
//
//     fn remove_node(&mut self, data: Self::NodeId) -> Option<Self::NodeData> {
//         todo!()
//     }
//
//     fn add_edge(
//         &mut self,
//         node_from: Self::NodeId,
//         node_to: Self::NodeId,
//         data: &Self::EdgeData,
//       ) -> Self::EdgeId {
//         todo!()
//     }
//
//     fn remove_edge(&mut self, data: Self::EdgeId) -> Option<Self::EdgeData> {
//         todo!()
//     }
// }
//
// // pub struct NoteGraph {
// //     pub nodes: HashMap<NodeId, Node>,
// //     pub adjacency: HashMap<NodeId, Vec<AAdjacement<NodeId, Edge>>>
// // }
//
// // impl<'a> Graph<'a> for NoteGraph {
// //     type NodeId = NodeId;
// //     type NodeData = NodeData;
//   
// //     type EdgeId = EdgeId;
// //     type EdgeData = EdgeData;
//   
// //     type NodeIter = NodeIter<'a>;
// //     type NodeIterMut = NodeIterMut<'a>;
//   
// //     type EdgeIter = EdgeIter<'a>;
// //     type EdgeIterMut = EdgeIterMut<'a>;
//   
// //     fn iter_nodes(&self) -> Self::NodeIter {
// //       NODES_ITER
// //     }
// //     fn iter_nodes_mut(&mut self) -> Self::NodeIterMut {
// //       unimplemented!()
// //     }
//   
// //     fn iter_edges(&self) -> Self::EdgeIter {
// //       EDGES_ITER
// //     }
// //     fn iter_edges_mut(&mut self) -> Self::EdgeIterMut {
// //       unimplemented!()
// //     }
//   
// //     /// Iterate edges incident to the node
// //     fn iter_incidents(&self, _node_id: Self::NodeId) -> Self::EdgeIter {
// //       unimplemented!()
// //     }
// //     /// Iterate edges incident to the node (mutably)
// //     fn iter_incidents_mut(&mut self, _node_id: Self::NodeId) -> Self::EdgeIterMut {
// //       unimplemented!()
// //     }
//   
// //     fn get_node(&self, id: Self::NodeId) -> &'a Self::NodeData {
// //       NODES
// //         .iter()
// //         .find(|node| node.0 == id)
// //         .map(|(_, node)| node)
// //         .unwrap()
// //     }
// //     fn get_node_mut(&mut self, _id: Self::NodeId) -> &'a mut Self::NodeData {
// //       unimplemented!()
// //     }
//   
// //     fn get_edge(&self, id: Self::EdgeId) -> &'a Self::EdgeData {
// //       EDGES
// //         .iter()
// //         .find(|edge| edge.0 == id)
// //         .map(|(_, edge)| edge)
// //         .unwrap()
// //     }
// //     fn get_edge_mut(&mut self, _id: Self::EdgeId) -> &'a mut Self::EdgeData {
// //       unimplemented!()
// //     }
// //     /// Get incident nodes of the edge
// //     fn get_edge_incidents(&self, edge_id: Self::EdgeId) -> EdgeIncidents<Self::NodeId> {
// //       EDGES
// //         .iter()
// //         .find(|edge| edge.0 == edge_id)
// //         .map(|(EdgeId(node1_id, node2_id), ..)| EdgeIncidents {
// //           node_from: *node1_id,
// //           node_to: *node2_id,
// //         })
// //         .unwrap()
// //     }
//   
// //     /// Adds a node to the graph
// //     fn add_node(&mut self, _data: &Self::NodeData) -> Self::NodeId {
// //       unimplemented!()
// //     }
//   
// //     /// Removes a node from the graph, returning it's `NodeData` if the node was previously in the graph
// //     fn remove_node(&mut self, _data: Self::NodeId) -> Option<Self::NodeData> {
// //       unimplemented!()
// //     }
//   
// //     /// Adds an edge to the graph
// //     fn add_edge(
// //       &mut self,
// //       _node_from: Self::NodeId,
// //       _node_to: Self::NodeId,
// //       _data: &Self::EdgeData,
// //     ) -> Self::EdgeId {
// //       unimplemented!()
// //     }
//   
// //     /// Removes an edge from the graph, returning it's `EdgeData` if the edge was previously in the graph
// //     fn remove_edge(&mut self, _data: Self::EdgeId) -> Option<Self::EdgeData> {
// //       unimplemented!()
// //     }
// //   }
