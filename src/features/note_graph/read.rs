use std::fs;

use pulldown_cmark::{Event, LinkType, Parser, Tag};

use super::{
  models::{Link, LinkEdge, LinkNode, LinkNodeId},
  note_graph::{AAdjacement, ANoteGraph},
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
  pub markdown: bool,
  // TODO full path
}

#[allow(dead_code)] // TODO remove
pub fn create_graph_default_options(
  pathes: std::slice::Iter<'_, String>,
) -> ANoteGraph<LinkNodeId, LinkEdge, LinkNode> {
  create_graph(
    pathes,
    |op: &Options| !op.url && !op.image && op.markdown,
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
) -> ANoteGraph<LinkNodeId, LinkEdge, LinkNode> {
  let mut graph_draft_v1: Vec<(FilePath, Vec<Link>)> = vec![];
  // let mut graph_draft_v2: HashMap<FilePath, Vec<Link>> = HashMap::default();

  let mut graph: ANoteGraph<LinkNodeId, LinkEdge, LinkNode> = ANoteGraph::default();

  #[derive(Debug, Clone, PartialEq, Hash, Eq)]
  struct FilePath {
    basename: String,
    dirname: String,
  }

  impl FilePath {
      fn fullname(&self) -> String {
          format!("{}/{}", self.dirname, self.basename)
      }
  }

  // make nodes from paths
  let mut new_paths: Vec<FilePath> = vec![];
  for path in pathes {
    if !path.starts_with("/") {
      log::warn!(
        "Oaoaoaoao, a global path has not been detected, code is red, code is red: {}",
        path
      );
      continue;
    }

    match fs::metadata(path) {
      Ok(metadata) => {
        if !metadata.is_file() {
          log::warn!("Oaoaoaoao, is not a files: {}", path);
        }

        // if !path.ends_with(".md") { // TODO think
        //   log::warn!("Oaoaoaoao, is not a markdown file: {}", path);
        // }
      }
      Err(_) => {
        log::warn!("Oaoaoaoao, file not found: {}", path);
      }
    }

    let splited_path = path.split("/").collect::<Vec<&str>>();
    let path = FilePath {
      basename: splited_path.last().unwrap().to_string(),
      dirname: splited_path[..splited_path.len() - 1].join("/"),
    };

    // add root node(s)
    graph.nodes.insert(
      LinkNodeId(path.fullname()),
      LinkNode {
        is_image: false,
      },
    );

    new_paths.push(path);
  }

  for path in new_paths { // TODO check full path
    let links = links_from_path(path.fullname().as_str());
    let mut filtred_links = vec![];

    for link in links.iter() {
      if link_type(link.r#type) {
        let re = regex::Regex::new(r"[a-z]+://").unwrap();

        let option = Options {
          url: re.is_match(link.destination.as_str()),
          image: link.is_image,
          markdown: link.destination.ends_with(".md"),
        };

        if options(&option) {
          // normalize name
          let mut name = link.normalized_name.clone();
          if !option.url && link.destination.starts_with("./") {
            name = link.destination[2..].to_string();
          }

          filtred_links.push(Link {
            normalized_name: name,
            ..(*link).clone()
          })
        }
      }
    }

    graph_draft_v1.push((path.clone(), filtred_links.clone()));
    // graph_draft_v2.insert(path, filtred_links);
  }

  // make edges from links
  for (path, links) in graph_draft_v1.iter() {
    for link in links.iter() {
      let node_id = LinkNodeId(link.normalized_name.clone());

      graph.nodes.insert(
        node_id.clone(),
        LinkNode {
          is_image: link.is_image,
        },
      );

      let path_fullname = path.fullname();
      let node_id_by_path = LinkNodeId(path_fullname);

      match graph.adjacency.get::<LinkNodeId>(&node_id_by_path) {
        Some(adjacents) => {
          let mut new_adjacents = adjacents.clone();
          new_adjacents.push(AAdjacement(
            node_id,
            LinkEdge {
              link_type: link.r#type,
              text: link.text.clone(),
              title: link.title.clone(),
            },
          ));
          graph.adjacency.insert(node_id_by_path, new_adjacents.clone());
        }
        None => {
          graph.adjacency.insert(
            node_id_by_path,
            vec![AAdjacement(
              node_id,
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
            log::debug!("{:?}", r#type);
            println!("{:?}", r#type);
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
            LinkType::Inline | LinkType::Reference | LinkType::Shortcut | LinkType::Collapsed | LinkType::Autolink => {}
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
  fn linktypes() {
    let pathes = vec![MARKDOWN_TEST.to_string()];
    let graph = super::create_graph_default_options(pathes.iter());
    println!("{:#?}", graph);
  }

  #[test]
  fn path_test() {
    let a = Path::new("./name.md");
    let b = Path::new("name.md");

    assert_ne!(a, b)
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
