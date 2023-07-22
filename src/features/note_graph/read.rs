use std::fs;

use egui::epaint::ahash::HashMap;
use pulldown_cmark::{Event, LinkType, Parser, Tag};

use super::{
  models::{Link, LinkEdge, LinkNode},
  note_graph::{Adjacement, Node, NoteGraph},
};

fn work_space() -> String {
  std::env::current_dir()
    .unwrap()
    .as_path()
    .to_str()
    .unwrap()
    .to_string()
}

#[derive(Default)]
pub struct Options {
  pub image: bool,
  pub url: bool,
  pub mardown: bool,
  // TODO full path
}

pub fn create_graph_default_options(
  pathes: std::slice::Iter<'_, String>,
) -> NoteGraph<String, LinkEdge, LinkNode> {
  create_graph(
    pathes,
    |op: &Options| !op.url && !op.image && op.mardown,
    |link_type: LinkType| {
      link_type == LinkType::Inline
        || link_type == LinkType::Reference
        || link_type == LinkType::Shortcut
        || link_type == LinkType::Collapsed
    },
  )
}

pub fn create_graph(
  pathes: std::slice::Iter<'_, String>,
  options: impl Fn(&Options) -> bool,
  link_type: impl Fn(LinkType) -> bool,
) -> NoteGraph<String, LinkEdge, LinkNode> {
  let mut graph_draft_v1: Vec<(String, Vec<Link>)> = vec![];
  let mut graph_draft_v2: HashMap<String, Vec<Link>> = HashMap::default();

  let mut graph: NoteGraph<String, LinkEdge, LinkNode> = NoteGraph::default();

  // make nodes from paths
  let mut new_paths = vec![];
  for path in pathes {
    // normalize name
    let mut name = path.clone();
    if path.starts_with("./") {
      name = path[2..].to_string();
    }
    // else if path.starts_with("/") {
    //   // TODO handle global path
    //   log::warn!(
    //     "Oaoaoaoao, a global path has been detected, code is red, code is red: {}",
    //     path
    //   );
    //   continue;
    // }

    // add root node(s)
    graph.nodes.insert(
      name.clone(),
      LinkNode {
        id: name.clone(),
        is_image: false,
      },
    );

    new_paths.push(name);
  }

  for path in new_paths {
    let links = links_from_path(path.as_str());
    let mut filtred_links = vec![];

    for link in links.iter() {
      if link_type(link.r#type) {
        let re = regex::Regex::new(r"[a-z]+://").unwrap();

        let option = Options {
          url: re.is_match(link.destination.as_str()),
          image: link.is_image,
          mardown: link.destination.ends_with(".md"),
        };

        if options(&option) {
          // normalize name
          let mut name = link.normalized_name.clone();
          if !option.url {
            if link.destination.starts_with("./") {
              name = link.destination[2..].to_string();
            }
          }

          filtred_links.push(Link {
            normalized_name: name,
            ..(*link).clone()
          })
        }
      }
    }

    graph_draft_v1.push((path.clone(), filtred_links.clone()));
    graph_draft_v2.insert(path, filtred_links);
  }

  // make edges from links
  for (path, links) in graph_draft_v1.iter() {
    // TODO rewrite to graph_draft_v2
    for link in links.iter() {
      graph.nodes.insert(
        link.normalized_name.clone(),
        LinkNode {
          id: link.normalized_name.clone(),
          is_image: link.is_image,
        },
      );

      match graph.adjacency.get::<String>(path) {
        Some(adjacents) => {
          let mut new_adjacents = adjacents.clone();
          new_adjacents.push(Adjacement(
            link.normalized_name.clone(),
            LinkEdge {
              link_type: link.r#type,
              text: link.text.clone(),
              title: link.title.clone(),
            },
          ));
          graph.adjacency.insert(path.clone(), new_adjacents.clone());
        }
        None => {
          graph.adjacency.insert(
            path.clone(),
            vec![Adjacement(
              link.normalized_name.clone(),
              LinkEdge {
                link_type: link.r#type,
                text: link.text.clone(),
                title: link.title.clone(),
              },
            )],
          );
        }
      }
    }
  }

  return graph;
}

// TODO front matter (make option)
#[allow(dead_code)]
pub fn links_from_path(path: &str) -> Vec<Link> {
  let markdown = fs::read_to_string(format!("{}{}", work_space(), path))
    .expect("Failed to read the Markdown file");

  let parser = Parser::new(&markdown);

  let mut link_title = String::new();
  let mut in_link = false;

  let mut links: Vec<Link> = vec![];
  for event in parser {
    match event {
      Event::Start(Tag::Link(r#type, _, _) | Tag::Image(r#type, _, _)) => {
        link_title.clear();
        match r#type {
          LinkType::Inline | LinkType::Reference | LinkType::Shortcut | LinkType::Collapsed => {
            in_link = true;
          }
          _ => continue,
        };
      }
      Event::End(tag @ Tag::Link(..) | tag @ Tag::Image(..)) => {
        in_link = false;
        if let Tag::Link(r#type, destination, title) | Tag::Image(r#type, destination, title) =
          tag.clone()
        {
          match r#type {
            LinkType::Inline | LinkType::Reference | LinkType::Shortcut | LinkType::Collapsed => {}
            LinkType::Autolink => {}
            _ => continue,
          };
          links.push(Link::new(
            r#type,
            &link_title,
            &destination,
            &title,
            if let Tag::Image(..) = tag {
              true
            } else {
              false
            },
          ));
        }
      }
      Event::Text(text) if in_link => {
        link_title.push_str(&text.to_string());
      }
      _ => continue,
    }
  }

  links
}

#[cfg(test)]
mod test {
  use std::{fs, path::Path};

  use markdown_parser::{read_file, Error};
  use pulldown_cmark::{Event, Parser, Tag};

  use crate::features::note_graph::read::work_space;

  use super::links_from_path;

  const MARKDOWN_TEST: &str = "/assets/markdown/sample.md";

  #[test]
  fn graph_from_one_file() {
    let pathes = vec![MARKDOWN_TEST.to_string()];
    let graph = super::create_graph_default_options(pathes.iter());
    println!("{:#?}", graph);
  }

  #[test]
  fn markdown_rust_test() -> Result<(), Error> {
    let path = format!("{}{}", work_space(), MARKDOWN_TEST);
    println!("{}", path);
    let md = read_file(path)?;
    let content = md.content();
    println!("{}", content);
    Ok(())
  }

  #[test]
  fn path_test() {
    let a = Path::new("./name.md");
    let b = Path::new("name.md");

    assert_eq!(a, b)
  }

  #[test]
  fn pulldown_cmark_test() {
    println!("{:#?}", links_from_path(MARKDOWN_TEST));
  }

  #[test]
  fn pulldown_cmark_test_2() {
    let markdown = fs::read_to_string(format!("{}{}", work_space(), MARKDOWN_TEST))
      .expect("Failed to read the Markdown file");
    let parser = Parser::new(&markdown);

    let mut link_text = String::new();
    let mut in_link = false;

    // Iterate over each event from the parser
    for event in parser {
      match event {
        Event::Start(Tag::Link(r#type, url, _title)) => {
          match r#type {
            pulldown_cmark::LinkType::Inline | pulldown_cmark::LinkType::Reference => {
              in_link = true;
            }
            pulldown_cmark::LinkType::Autolink => {}
            _ => continue, // (LinkType::ReferenceUnknown, LinkType::Collapsed, etc.)
          };
          println!("Link Url: {}", url.to_string());
          link_text.clear();
        }
        Event::End(Tag::Link(_, _, _)) => {
          in_link = false;
        }
        Event::Text(text) if in_link => {
          link_text.push_str(&text.to_string());
          println!("Link Text: {}", link_text.to_string());
        }
        _ => continue,
      }
    }
  }
}
