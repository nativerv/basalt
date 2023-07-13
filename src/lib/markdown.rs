use pulldown_cmark::{Event, LinkType, Parser, Tag};
use std::fs;

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

#[derive(Debug)]
pub struct Link {
  pub r#type: LinkType,
  pub text: String,
  pub destination: String,
  pub title: String,
}

impl Link {
  pub fn new(r#type: LinkType, text: &str, destination: &str, title: &str) -> Self {
    Link {
      r#type,
      text: text.to_owned(),
      destination: destination.to_owned(),
      title: title.to_owned(),
    }
  }
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
      Event::Start(Tag::Link(r#type, _, _)
        | Tag::Image(r#type, _, _)) => {
        link_title.clear();
        match r#type {
          LinkType::Inline | LinkType::Reference | LinkType::Shortcut | LinkType::Collapsed => {
            in_link = true;
          }
          _ => continue,
        };
      }
      Event::End(
        Tag::Link(r#type, destination, title)
        | Tag::Image(r#type, destination, title)
      ) => {
        in_link = false;
        match r#type {
          LinkType::Inline
          | LinkType::Reference
          | LinkType::Shortcut
          | LinkType::Collapsed => {}
          LinkType::Autolink => {}
          _ => continue,
        };
        links.push(Link::new(r#type, &link_title, &destination, &title));
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
  use std::fs;

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
