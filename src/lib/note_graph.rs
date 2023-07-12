use std::collections::HashMap;

pub struct NoteGraph<NodeId, Edge = (), Node = ()> {
    pub nodes: HashMap<NodeId, Node>,
    pub adjacency: HashMap<NodeId, Vec<Adjacement<NodeId, Edge>>>
}

pub struct  Adjacement<NodeId, Edge>(NodeId, Edge);

pub struct Node {
    pub link: String,
}

pub struct Edge {
    pub text: String,
}