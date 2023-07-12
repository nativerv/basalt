use pulldown_cmark::{Parser, Event, Tag};
use std::fs;

fn work_space() -> String {
    std::env::current_dir().unwrap().as_path().to_str().unwrap().to_string()
}

pub fn links_from_path(path: &str) -> Vec<String> {
    let markdown = fs::read_to_string(format!("{}{}", work_space(), path))
        .expect("Failed to read the Markdown file");
    let parser = Parser::new(&markdown);

    let mut links = vec![];
    for event in parser {
        if let Event::Start(Tag::Link(link_type, url, _)) = event {
            let link_url = match link_type {
                pulldown_cmark::LinkType::Inline | pulldown_cmark::LinkType::Reference => url.into_string(),
                pulldown_cmark::LinkType::Autolink => format!("{}", url),
                _ => continue, // (LinkType::ReferenceUnknown, LinkType::Collapsed, etc.)
            };

            links.push(link_url);
        }
    }

    links
}

pub fn local_links_by_path(path: &str) -> Vec<String> {
    links_from_path(path).iter()
        .filter(|link| {! link.starts_with("http://") && ! link.starts_with("https://")})
        .map(|link| {link.to_owned()})
        .collect::<Vec<String>>()
}

pub fn url_by_path(path: &str) -> Vec<String> {
    links_from_path(path).iter()
        .filter(|link| {link.starts_with("http://") || link.starts_with("https://")})
        .map(|link| {link.to_owned()})
        .collect::<Vec<String>>()
}

#[cfg(test)]
mod test {
    use markdown_parser::{
        read_file, Error
    };

    use crate::markdown::work_space;

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
    }
}