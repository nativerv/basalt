use pulldown_cmark::LinkType;

use super::note_graph::{Node, Edge};

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

impl Node for LinkNode {
    
}

#[derive(Debug, Clone)]
pub struct LinkEdge {
  pub link_type: LinkType,
  pub text: String,
  pub title: String,
}

impl Edge for LinkEdge { }

