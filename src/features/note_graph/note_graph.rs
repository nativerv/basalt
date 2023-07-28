use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct NoteGraph<NodeId, Edge = (), Node = ()> {
    pub nodes: HashMap<NodeId, Node>,
    pub adjacency: HashMap<NodeId, Vec<Adjacement<NodeId, Edge>>>
}

impl<NodeId, E, N> Default for NoteGraph<NodeId, E, N>
where
    NodeId: std::hash::Hash + Eq,
    N: Node,
    E: Edge,
{
    fn default() -> Self {
        NoteGraph {
            nodes: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct  Adjacement<NodeId, Edge>(pub NodeId, pub Edge);

pub trait Edge {
}

pub trait Node {

}
