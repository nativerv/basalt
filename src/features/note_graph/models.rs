use pulldown_cmark::LinkType;

use super::note_graph::{ANode, AEdge};

#[derive(Debug, Clone)]
pub struct Link {
  pub r#type: LinkType,
  pub text: String,
  pub destination: String,
  pub title: String,
  pub is_image: bool,
  pub normalized_name: String, 
}

impl Link {
  pub fn new(r#type: LinkType, text: &str, destination: &str, title: &str, is_image: bool) -> Self {
    Link {
      r#type,
      text: text.to_owned(),
      destination: destination.to_owned(),
      title: title.to_owned(),
      is_image: is_image.to_owned(),
      normalized_name: "".into(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct LinkNode {
  pub is_image: bool,
}

impl ANode for LinkNode { }

#[derive(Debug, Clone)]
pub struct LinkEdge {
  // line
  pub link_type: LinkType,
  pub text: String,
  pub title: String,
}

impl AEdge for LinkEdge { }

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct LinkNodeId (pub String);
