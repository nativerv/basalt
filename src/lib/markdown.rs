use pulldown_cmark::{Event, LinkType, Parser, Tag};
use std::{arch::x86_64::_MM_MASK_INVALID, default, fs};

use super::{graph::Graph, note_graph::NoteGraph};

fn work_space() -> String {
  std::env::current_dir()
    .unwrap()
    .as_path()
    .to_str()
    .unwrap()
    .to_string()
}

// pub fn req(path: &str) {
//   let links = local_links_by_path(path);
// }

#[derive(Debug, Clone)]
pub struct Link {
  pub r#type: LinkType,
  pub text: String,
  pub destination: String,
  pub title: String,
  pub is_image: bool,

  pub normalized_name: String, // ./path/to/file 

  // /home/yukkop/pidor.md

  // ./pidor.md
  // pidor.md
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

#[derive(Default)]
pub struct Options {
  pub image: bool,
  pub url: bool,
  pub mardown: bool,
  // TODO full path
}

// (url & markdown & !image) | (path & image)
// Option { url: true, marcdown: true, image: false, .. } | Option { path: true, image: true }

pub fn create_graph(
  pathes: std::slice::Iter<'_, String>,
  options: impl Fn(&Options) -> bool,
  link_type: impl Fn(LinkType) -> bool,
) {
  let mut result = vec![];

  for path in pathes {
    let links = links_from_path(path);

    for link in links.iter() {

      if link_type(link.r#type) {
        let re = regex::Regex::new(r"[a-z]+://").unwrap();

        let option = Options {
          url: re.is_match(link.destination.as_str()),
          image: link.is_image,
          mardown: link.destination.ends_with(".md"),
        };

        if options(&option) {
          let mut name = link.normalized_name.clone();
          if !option.url {
            if link.destination.starts_with("./") {
              name = link.destination[2..].to_string();
            }
          }
          result.push(
            Link {
              normalized_name: name,
              ..(*link).clone()
            }
          )
        }
      }
    }
  }

  let graph: NoteGraph<String, (), Link>;
  // TODO -> up
  // for link in result {
  //   graph.nodes.insert(link.normalized_name, link)
  // }
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

// pub fn local_links_by_path(path: &str) -> Vec<String> {
//   links_from_path(path)
//     .iter()
//     .filter(|link| !link.starts_with("http://") && !link.starts_with("https://"))
//     .map(|link| link.to_owned())
//     .collect::<Vec<String>>()
// }

// pub fn url_by_path(path: &str) -> Vec<String> {
//   links_from_path(path)
//     .iter()
//     .filter(|link| link.starts_with("http://") || link.starts_with("https://"))
//     .map(|link| link.to_owned())
//     .collect::<Vec<String>>()
// }

#[cfg(test)]
mod test {
  use std::{fs, path::Path};

  use markdown_parser::{read_file, Error};
  use pulldown_cmark::{Event, Parser, Tag};

  use crate::lib::markdown::work_space;

  use super::links_from_path;

  const MARKDOWN_TEST: &str = "/assets/markdown/sample.md";

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
    let a = Path::new("./pidor.md");
    let b = Path::new("pidor.md");

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
