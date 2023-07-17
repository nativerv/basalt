use std::collections::HashMap;

pub struct NoteGraph<NodeId, Edge = (), Node = ()> {
    pub nodes: HashMap<NodeId, Node>,
    pub adjacency: HashMap<NodeId, Vec<Adjacement<NodeId, Edge>>>
}

impl<NodeId, Edge, Node> Default for NoteGraph<NodeId, Edge, Node>
where
    NodeId: std::hash::Hash + Eq,
{
    fn default() -> Self {
        NoteGraph {
            nodes: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }
}

pub struct  Adjacement<NodeId, Edge>(NodeId, Edge);

pub struct Node {
    pub link: String,
}

pub struct Edge {
    pub text: String,
}