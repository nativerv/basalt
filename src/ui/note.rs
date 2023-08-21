use std::cell::RefCell;
use std::rc::Rc;

use egui::{
  text::LayoutJob, vec2, Align, Color32, FontFamily, FontId, Layout, ScrollArea, Stroke, TextEdit,
  TextFormat, TextStyle, Ui,
};
use egui::{Sense, Separator};
use egui_extras::image::RetainedImage;
use pulldown_cmark::{Event, HeadingLevel, LinkType, Tag};

use crate::lib::images_cache::ImagesCache;
use crate::ui::Checkbox;

use crate::features::note_preview::NoteData;

/// Define a struct to manage the UI of the note preview
pub struct Note {
  note: Rc<RefCell<NoteData>>,
  font_size: f32,
  list_indent_lvl: u8,
  list_indent_strenght: u8,
}

impl Note {
  /// Create a new Note instance
  pub fn new(note: Rc<RefCell<NoteData>>) -> Self {
    Self {
      note,
      font_size: 16.0,
      list_indent_lvl: 0,
      list_indent_strenght: 4,
    }
  }
}
  

/// Define a struct to represent a link
struct Link {
  url: String,
  link_type: LinkType,
  is_image: bool,
}

// Default implementation for the Link struct
impl Default for Link {
  fn default() -> Self {
    Self {
      url: "".to_string(),
      link_type: LinkType::Inline,
      is_image: false,
    }
  }
}

// Define an enum to represent the kinds of items in the note
enum ItemKind {
  Text,
  Link { url: String, is_image: bool },
  Separator,
  ListItem { indent_size: u8 },
  Spacing,
  Indent { indent_size: u8 },
  Checkbox { checked: bool },
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


// Implementation for the NotePreviewUi struct
impl Note {
  /// Render the UI for the note preview
  pub fn ui(&mut self, ui: &mut Ui) {
    ui.heading("Note Preview");
    self.render_markdown(ui);

  }

  // Render the markdown content
  fn render_markdown(&mut self, ui: &mut Ui) {
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
    for event in &mut self.note.borrow().parsing_note() {
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
            current_text_style.strikethrough = Stroke::new(1.0, code_text_style.color);
          }
          // Handle link tags
          Tag::Link(link_type, url, _) => {
            link.url = url.to_string();
            link.link_type = link_type.clone();
            is_link = true;
          }
          Tag::Image(link_type, url, _) => {
            link.url = url.to_string();
            link.link_type = link_type.clone();
            link.is_image = true;
            is_link = true;
            items.push(Item::new(LayoutJob::default(), ItemKind::Spacing));
          }
          Tag::List(..) => {
            self.list_indent_lvl += 1;
            items.push(Item::new(LayoutJob::default(), ItemKind::Spacing));
          }
          Tag::Item => {
            items.push(Item::new(
              LayoutJob::default(),
              ItemKind::Indent {
                indent_size: (self.list_indent_strenght * (self.list_indent_lvl - 1)),
              },
            ));
            items.push(Item::new(
              LayoutJob::default(),
              ItemKind::ListItem {
                indent_size: self.list_indent_strenght,
              },
            ));
          }
          _ => {
            //println!("Start: {:?}", tag)
          }
        },
        // Handle end tags
        Event::End(tag) => match tag {
          // Handle heading end tags
          Tag::Heading(_, _, _) => {
            current_text_style.font_id.size = self.font_size;
            items.push(Item::new(LayoutJob::default(), ItemKind::Spacing));
          }
          // Handle paragraph end tags
          Tag::Paragraph => items.push(Item::new(LayoutJob::default(), ItemKind::Spacing)),
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
            items.push(Item::new(LayoutJob::default(), ItemKind::Spacing));
          }
          // Handle link end tags
          Tag::Link(_, _, _) => {
            is_link = false;
            link = Link::default();
          }
          Tag::Image(_link_type, _url, _) => {
            link = Link::default();
            is_link = false;
            items.push(Item::new(LayoutJob::default(), ItemKind::Spacing));
          }
          Tag::Item => {
            items.push(Item::new(LayoutJob::default(), ItemKind::Spacing));
          }
          Tag::List(..) => {
            self.list_indent_lvl -= 1;
          }
          _ => {
            //println!("End: {:?}", tag)
          }
        },
        Event::Html(s) => {}//println!("Html: {:?}", s),
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
                url: link.url.as_str().to_string(),
                is_image: link.is_image,
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
        Event::FootnoteReference(s) => {}//println!("FootnoteReference: {:?}", s),
        Event::TaskListMarker(b) => items.push(Item::new(
          LayoutJob::default(),
          ItemKind::Checkbox { checked: b },
        )),
        Event::SoftBreak => {
          layout_job.append(" ", 0.0, current_text_style.clone());
        }
        Event::HardBreak => {
          items.push(Item::new(LayoutJob::default(), ItemKind::Spacing));
        }
        Event::Rule => {
          items.push(Item::new(LayoutJob::default(), ItemKind::Separator));
          items.push(Item::new(LayoutJob::default(), ItemKind::Spacing));
        }
      }
    }

    // Define the layout for rendering items
    let layout = Layout::left_to_right(Align::BOTTOM).with_main_wrap(true);
    let initial_size = vec2(
      ui.available_width(),
      ui.spacing().interact_size.y, // Assume there will be
    );

    // Render items in a scrollable area
    ScrollArea::vertical().id_source("renderer").show(ui, |ui| {
      ui.allocate_ui_with_layout(initial_size, layout, |ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let row_height = ui.text_style_height(&TextStyle::Body);
        ui.set_row_height(row_height);

        // Render each item
        for item in items {
          Self::item_render(self, ui, &item);
        }
      });
    });
  }

  // Render an individual item
  fn item_render(&mut self, ui: &mut Ui, item: &Item) {
    match &item.kind {
      // Render text items
      ItemKind::Text => {
        ui.label(item.layout_job.clone());
      }
      // Render link items
      ItemKind::Link { url, is_image, .. } => {
        if *is_image {
          let mut note_data = self.note.borrow_mut();
          let image_bytes = note_data.images_cache.load_image(url.as_str());
          match image_bytes {
            Ok(image_bytes) => {
              let image = RetainedImage::from_image_bytes(url.clone(), &image_bytes);
              match image {
                Ok(image) => {
                  image.show_max_size(
                    ui,
                    vec2(
                      ui.available_rect_before_wrap().width() - 10.0,
                      f32::INFINITY,
                    ),
                  );
                }
                Err(_) => {
                  ui.label(item.layout_job.clone());
                }
              }
            }
            Err(_) => {
              ui.label(item.layout_job.clone());
            }
          }
        } else {
          if ui.add(egui::Link::new(item.layout_job.clone())).clicked() {
            println!("Link clicked to url: {url}");
          };
        }
      }
      ItemKind::Separator => {
        ui.add(Separator::default().horizontal());
      }
      ItemKind::ListItem { indent_size } => {
        let row_height = ui.text_style_height(&TextStyle::Body);
        let one_indent = row_height / 2.0;
        let (rect, _response) = ui.allocate_exact_size(
          vec2(
            <u8 as Into<f32>>::into(*indent_size) * one_indent,
            row_height,
          ),
          Sense::hover(),
        );
        ui.painter().circle_filled(
          rect.center(),
          rect.height() / 8.0,
          ui.visuals().strong_text_color(),
        );
      }
      ItemKind::Spacing => {
        self.vertical_empty_separator(ui);
      }
      ItemKind::Indent { indent_size } => {
        self.indent(ui, *indent_size);
      }
      ItemKind::Checkbox { checked } => {
        ui.add(Checkbox::without_text(*checked).interactive(false));
      }
    }
  }

  fn vertical_empty_separator(&self, ui: &mut Ui) {
    let row_height: f32 = ui.text_style_height(&TextStyle::Body);
    ui.allocate_exact_size(vec2(0.0, row_height), Sense::hover()); // make sure we take up some height
    ui.end_row();
    ui.set_row_height(row_height);
  }

  fn indent(&self, ui: &mut Ui, indent_size: u8) {
    let row_height: f32 = ui.text_style_height(&TextStyle::Body);
    let one_indent = row_height / 2.0;
    ui.allocate_exact_size(
      vec2(
        <u8 as Into<f32>>::into(indent_size) * one_indent,
        row_height,
      ),
      Sense::hover(),
    );
  }
}
