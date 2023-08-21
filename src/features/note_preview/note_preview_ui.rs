// Import necessary modules
use egui::{
  text::LayoutJob, vec2, Align, Color32, FontFamily, FontId, Hyperlink, Layout, ScrollArea, Stroke, TextEdit, TextFormat, TextStyle, Ui,
};
use pulldown_cmark::{Event, HeadingLevel, LinkType, Tag};

// Import required types from the current crate
use crate::features::note_preview::NotePreview;

// Define a struct to manage the UI of the note preview
pub struct NotePreviewUi {
  note: NotePreview,
  font_size: f32,
}

// Define a struct to represent a link
struct Link {
  url: String,
  link_type: LinkType,
}

// Default implementation for the Link struct
impl Default for Link {
  fn default() -> Self {
      Self {
          url: "".to_string(),
          link_type: LinkType::Inline,
      }
  }
}

// Define an enum to represent the kinds of items in the note
enum ItemKind {
  Text,
  Link { url: String, link_type: LinkType },
}

// Default implementation for the ItemKind enum
impl Default for ItemKind {
  fn default() -> Self {
      ItemKind::Text
  }
}

// Define a struct to represent an item in the note
#[derive(Default)]
struct Item {
  layout_job: LayoutJob,
  kind: ItemKind,
}

// Implementation for the Item struct
impl Item {
  // Create a new Item instance with the provided layout job and kind
  fn new(layout_job: LayoutJob, kind: ItemKind) -> Self {
      Self { layout_job, kind }
  }
}

// Default implementation for the NotePreviewUi struct
impl Default for NotePreviewUi {
  fn default() -> Self {
      Self {
          font_size: 14.0,
          note: NotePreview::default(),
      }
  }
}

// Implementation for the NotePreviewUi struct
impl NotePreviewUi {
  // Render the UI for the note preview
  pub fn ui(&mut self, ui: &mut Ui) {
      ui.heading("Note Preview");
      ui.columns(2, |columns| {
          self.render_markdown_input(&mut columns[0]);
          self.render_markdown(&mut columns[1]);
      });
  }

  // Render the input area for markdown text
  fn render_markdown_input(&mut self, ui: &mut Ui) {
      ScrollArea::vertical().show(ui, |ui| {
          ui.add(
              TextEdit::multiline(&mut self.note.markdown_input)
                  .desired_width(f32::INFINITY)
                  .min_size(vec2(0.0, 300.0))
                  .id_source("source"),
          );
      });
  }

  // Render the markdown content
  fn render_markdown(&self, ui: &mut Ui) {
      // Initialize variables to track text styles and layout
      let mut current_text_style = TextFormat::default();
      let code_text_style = {
          let mut default = TextFormat::default();
          default.font_id = FontId::monospace(self.font_size);
          default.background = Color32::from_rgb(0, 0, 0);
          default
      };
      let mut items: Vec<Item> = Vec::new();
      let mut is_code = false;
      let mut is_link = false;
      let mut link = Link::default();
      let mut layout_job = LayoutJob::default();

      // Iterate through the markdown events
      for event in &mut self.note.parsing_note() {
          match event {
              // Handle start tags
              Event::Start(tag) => match tag {
                  // Handle heading tags
                  Tag::Heading(level, _, _) => {
                      current_text_style.font_id.size = match level {
                          HeadingLevel::H1 => self.font_size + 16.0,
                          HeadingLevel::H2 => self.font_size + 12.0,
                          HeadingLevel::H3 => self.font_size + 8.0,
                          HeadingLevel::H4 => self.font_size + 6.0,
                          HeadingLevel::H5 => self.font_size + 4.0,
                          HeadingLevel::H6 => self.font_size + 2.0,
                      };
                  }
                  // Handle paragraph tags
                  Tag::Paragraph => {
                      current_text_style.font_id.size = self.font_size;
                  }
                  // Handle code block tags
                  Tag::CodeBlock(_) => {
                      is_code = true;
                  }
                  // Handle strong text tags
                  Tag::Strong => {
                      current_text_style.color = ui.style().visuals.strong_text_color();
                  }
                  // Handle emphasis tags
                  Tag::Emphasis => {
                      current_text_style.italics = true;
                  }
                  // Handle strikethrough tags
                  Tag::Strikethrough => {
                      current_text_style.strikethrough =
                          Stroke::new(1.0, code_text_style.color);
                  }
                  // Handle link tags
                  Tag::Link(link_type, url, _) => {
                      link.url = url.to_string();
                      link.link_type = link_type.clone();
                      is_link = true;
                  }
                  _ => {
                      println!("Start: {:?}", tag)
                  }
              },
              // Handle end tags
              Event::End(tag) => match tag {
                  // Handle heading end tags
                  Tag::Heading(_, _, _) => {
                      current_text_style.font_id.size = self.font_size;
                      layout_job.append("\n", 0.0, current_text_style.clone());
                  }
                  // Handle paragraph end tags
                  Tag::Paragraph => {
                      layout_job.append("\n", 0.0, current_text_style.clone());
                  }
                  // Handle strong text end tags
                  Tag::Strong => {
                      current_text_style.color = ui.style().visuals.text_color();
                  }
                  // Handle emphasis end tags
                  Tag::Emphasis => {
                      current_text_style.italics = false;
                  }
                  // Handle strikethrough end tags
                  Tag::Strikethrough => {
                      current_text_style.strikethrough = Stroke::NONE;
                  }
                  // Handle code block end tags
                  Tag::CodeBlock(_) => {
                      is_code = false;
                  }
                  // Handle link end tags
                  Tag::Link(_, _, _) => {
                      is_link = false;
                      link = Link::default();
                  }
                  _ => {
                      println!("End: {:?}", tag)
                  }
              },
              Event::Html(s) => println!("Html: {:?}", s),
              Event::Text(s) => {
                  // Append text to the layout job with appropriate style
                  layout_job.append(
                      &s,
                      0.0,
                      if is_code {
                          code_text_style.clone()
                      } else {
                          current_text_style.clone()
                      },
                  );

                  // Push the current item to the items list
                  items.push(Item::new(
                      layout_job,
                      if is_link {
                          ItemKind::Link {
                              link_type: link.link_type,
                              url: link.url.as_str().to_string(),
                          }
                      } else {
                          ItemKind::Text
                      },
                  ));

                  // Reset the layout job for the next item
                  layout_job = LayoutJob::default();
              }
              Event::Code(s) => {
                  // Handle code text with appropriate style
                  current_text_style.font_id.family = FontFamily::Monospace;
                  current_text_style.background = Color32::from_rgb(0, 0, 0);
                  layout_job.append(&s, 0.0, current_text_style.clone());

                  // Push the current item to the items list
                  items.push(Item::new(layout_job, ItemKind::Text));

                  // Reset style and layout job
                  layout_job = LayoutJob::default();
                  current_text_style.background = Color32::TRANSPARENT;
                  current_text_style.font_id.family = FontFamily::Proportional;
              }
              Event::FootnoteReference(s) => println!("FootnoteReference: {:?}", s),
              Event::TaskListMarker(b) => println!("TaskListMarker: {:?}", b),
              Event::SoftBreak => {
                  layout_job.append(" ", 0.0, current_text_style.clone());
              }
              Event::HardBreak => {
                  layout_job.append("\n", 0.0, current_text_style.clone());
              }
              Event::Rule => println!("Rule"),
          }
      }

      // Define the layout for rendering items
      let layout = Layout::left_to_right(Align::BOTTOM).with_main_wrap(true);
      let initial_size = vec2(
          ui.available_width(),
          ui.spacing().interact_size.y, // Assume there will be
      );

      // Render items in a scrollable area
      ScrollArea::vertical()
          .id_source("renderer")
          .show(ui, |ui| {
              ui.allocate_ui_with_layout(initial_size, layout, |ui| {
                  ui.spacing_mut().item_spacing.x = 0.0;
                  let row_height = ui.text_style_height(&TextStyle::Body);
                  ui.set_row_height(row_height);

                  // Render each item
                  for item in items {
                      Self::item_render(ui, &item);
                  }
              });
          });
  }

  // Render an individual item
  fn item_render(ui: &mut Ui, item: &Item) {
      match &item.kind {
          // Render text items
          ItemKind::Text => {
              ui.label(item.layout_job.clone());
          }
          // Render link items
          ItemKind::Link { url, link_type } => {
              if ui
                  .add(Hyperlink::from_label_and_url(item.layout_job.clone(), url.clone()))
                  .clicked()
              {
                  println!("Link clicked to url: {url}" );
              }
              ;
          }
      }
  }
}
