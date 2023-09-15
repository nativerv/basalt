use crate::lib::publisher::Publisher;
use egui::{
  text::LayoutJob, vec2, Align, Color32, FontFamily, FontId, Layout, ScrollArea, Stroke,
  TextFormat, TextStyle, Ui,
};
use egui::{Sense, Separator};
use egui_extras::image::RetainedImage;
use pulldown_cmark::{Event, HeadingLevel, LinkType, Tag};
use std::cell::{Ref, RefCell};
use std::rc::Rc;

use crate::ui::Checkbox;

use crate::features::note_preview::NoteData;

/// Define a struct to manage the UI of the note preview
pub struct Note {
  note: Rc<RefCell<NoteData>>,
  font_size: f32,
  list_indent_lvl: u8,
  list_indent_strenght: u8,
  is_inside_list: bool,
  items: Vec<ItemKind>,
  is_updated: Rc<RefCell<bool>>,
  lists_stack: Vec<Option<u64>>,
}

impl Note {
  /// Create a new Note instance
  pub fn new(note: Rc<RefCell<NoteData>>) -> Self {
    let this_note = Self {
      note,
      font_size: 16.0,
      list_indent_lvl: 0,
      list_indent_strenght: 4,
      is_inside_list: false,
      items: Vec::new(),
      lists_stack: Vec::new(),
      is_updated: Rc::new(RefCell::new(true)),
    };
    let is_updated: Rc<RefCell<bool>> = Rc::clone(&this_note.is_updated);
    this_note
      .note
      .borrow_mut()
      .subscribe(Box::new(move || *is_updated.borrow_mut() = true));
    this_note
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
#[derive(Clone)]
enum ItemKind {
  Text {
    layout_job: LayoutJob,
  },
  Link {
    url: String,
    is_image: bool,
    layout_job: LayoutJob,
  },
  Separator,
  ListItem {
    indent_size: u8,
  },
  OrderedListItem {
    indent_size: u8,
    current_number: u64,
    layout_job: LayoutJob,
  },
  Spacing,
  ItemSpacing,
  Indent {
    indent_size: u8,
  },
  Checkbox {
    checked: bool,
  },
}

// Default implementation for the ItemKind enum
impl Default for ItemKind {
  fn default() -> Self {
    ItemKind::Text {
      layout_job: LayoutJob::default(),
    }
  }
}

// Implementation for the NotePreviewUi struct
impl Note {
  /// Render the UI for the note preview
  pub fn ui(&mut self, ui: &mut Ui) {
    if *self.is_updated.borrow() {
      self.regenerate_items(ui);
      *self.is_updated.borrow_mut() = false;
    }
    self.render(ui);
  }

  // Render the markdown content
  fn regenerate_items(&mut self, ui: &Ui) {
    self.items.clear();

    // Initialize variables to track text styles and layout
    let mut current_text_style = TextFormat::default();

    let code_text_style = {
      let mut default = TextFormat::default();
      default.font_id = FontId::monospace(self.font_size);
      default.background = Color32::from_rgb(0, 0, 0);
      default
    };

    let mut is_code = false;
    let mut is_link = false;
    let mut link = Link::default();
    let mut layout_job = LayoutJob::default();

    // Iterate through the markdown events
    for event in &mut Rc::clone(&self.note).borrow().parsing_note() {
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
            link.link_type = link_type;
            is_link = true;
          }
          Tag::Image(link_type, url, _) => {
            link.url = url.to_string();
            link.link_type = link_type;
            link.is_image = true;
            is_link = true;
            self.add_spacing(ItemKind::Spacing);
          }
          Tag::List(starting_number) => {
            println!("List: {:?}", tag);
            self.is_inside_list = true;
            self.list_indent_lvl += 1;
            self.lists_stack.push(starting_number);
            self.add_spacing(ItemKind::Spacing);
          }
          Tag::Item => {
            match &mut self.lists_stack.last_mut() {
              Some(Some(last_list)) => {
                layout_job.append(
                  &format!("{last_list}. "),
                  0.0,
                  if is_code {
                    code_text_style.clone()
                  } else {
                    current_text_style.clone()
                  },
                );
                self.items.push(ItemKind::OrderedListItem {
                  layout_job: layout_job,
                  current_number: last_list.clone(),
                  indent_size: (self.list_indent_strenght * (self.list_indent_lvl - 1)),
                });
                layout_job = LayoutJob::default();
                *last_list += 1;
              }
              Some(None) => {
                self.items.push(ItemKind::ListItem {
                  indent_size: (self.list_indent_strenght * (self.list_indent_lvl - 1)),
                });
                self.items.push(ItemKind::Indent {
                  indent_size: (self.list_indent_strenght),
                });
              }
              None => {
                log::error!("invariant: item in list without list");
              }
            }
            println!("Start Item: {:?}", tag)
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
            self.add_spacing(ItemKind::Spacing);
          }
          // Handle paragraph end tags
          Tag::Paragraph => {
            self.add_spacing(ItemKind::Spacing);
            println!("End Paragraph: {:?}", tag)
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
            self.add_spacing(ItemKind::Spacing);
          }
          // Handle link end tags
          Tag::Link(_, _, _) => {
            is_link = false;
            link = Link::default();
          }
          Tag::Image(_link_type, _url, _) => {
            link = Link::default();
            is_link = false;
            self.add_spacing(ItemKind::Spacing);
          }
          Tag::Item => {
            self.add_spacing(ItemKind::ItemSpacing);
            println!("End Item: {:?}", tag)
          }
          Tag::List(..) => {
            println!("End List: {:?}", tag);
            self.is_inside_list = false;
            self.list_indent_lvl -= 1;
            self.lists_stack.pop();
            if self.lists_stack.len() == 0 {
              while let Some(ItemKind::ItemSpacing) = self.items.last() {
                self.items.pop();
              }
              self.add_spacing(ItemKind::Spacing);
            }
          }
          _ => {
            //println!("End: {:?}", tag)
          }
        },
        Event::Html(_s) => {} //println!("Html: {:?}", s),
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
          self.items.push(if is_link {
            ItemKind::Link {
              url: link.url.as_str().to_string(),
              is_image: link.is_image,
              layout_job: layout_job,
            }
          } else {
            ItemKind::Text {
              layout_job: layout_job,
            }
          });
          // Reset the layout job for the next item
          layout_job = LayoutJob::default();
        }
        Event::Code(s) => {
          // Handle code text with appropriate style
          current_text_style.font_id.family = FontFamily::Monospace;
          current_text_style.background = Color32::from_rgb(0, 0, 0);
          layout_job.append(&s, 0.0, current_text_style.clone());

          // Push the current item to the items list
          self.items.push(ItemKind::Text { layout_job });

          // Reset style and layout job
          layout_job = LayoutJob::default();
          current_text_style.background = Color32::TRANSPARENT;
          current_text_style.font_id.family = FontFamily::Proportional;
        }
        Event::FootnoteReference(_s) => {} //println!("FootnoteReference: {:?}", s),
        Event::TaskListMarker(b) => self.items.push(ItemKind::Checkbox { checked: b }),
        Event::SoftBreak => {
          layout_job.append(" ", 0.0, current_text_style.clone());
        }
        Event::HardBreak => {
          self.add_spacing(ItemKind::Spacing);
        }
        Event::Rule => {
          self.items.push(ItemKind::Separator);
          self.add_spacing(ItemKind::Spacing);
        }
      }
    }
  }

  fn render(&mut self, ui: &mut Ui) {
    ui.heading("Note Preview");
    let layout = Layout::left_to_right(Align::BOTTOM).with_main_wrap(true);
    let initial_size = vec2(
      ui.available_width(),
      ui.spacing().interact_size.y, // Assume there will be
    );

    // Render items in a scrollable area
    ScrollArea::vertical()
      .id_source("renderer")
      .show_viewport(ui, |ui, _| {
        ui.allocate_ui_with_layout(initial_size, layout, |ui| {
          ui.spacing_mut().item_spacing.x = 0.0;
          let row_height = ui.text_style_height(&TextStyle::Body);
          ui.set_row_height(row_height);

          // Render each item
          for item in &self.items {
            self.item_render(ui, &item);
          }
        });
      });
  }

  // Render an individual item
  fn item_render(&self, ui: &mut Ui, item: &ItemKind) {
    match &item {
      // Render text items
      ItemKind::Text { layout_job } => {
        ui.label(layout_job.clone());
      }
      // Render link items
      ItemKind::Link {
        url,
        is_image,
        layout_job,
        ..
      } => {
        if *is_image {
          let mut note_data = self.note.borrow_mut();
          let image_bytes = note_data.images_cache.load_image(url.as_str());
          match image_bytes {
            Ok(image_bytes) => {
              let image = RetainedImage::from_image_bytes(url, &image_bytes);
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
                Err(_) => {}
              }
            }
            Err(_) => {}
          }
        } else {
          if ui.add(egui::Link::new(layout_job.clone())).clicked() {
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
        self.vertical_empty_separator(ui, 2);
      }
      ItemKind::ItemSpacing => {
        self.vertical_empty_separator(ui, 1);
      }
      ItemKind::Indent { indent_size } => {
        self.indent(ui, *indent_size);
      }
      ItemKind::Checkbox { checked } => {
        ui.add(Checkbox::without_text(*checked).interactive(false));
      }
      ItemKind::OrderedListItem {
        layout_job,
        indent_size,
        current_number: starting_number,
      } => {
        let row_height = ui.text_style_height(&TextStyle::Body);
        let one_indent = row_height / 2.0;
        let (rect, _response) = ui.allocate_exact_size(
          vec2(
            <u8 as Into<f32>>::into(*indent_size) * one_indent,
            row_height,
          ),
          Sense::hover(),
        );
        ui.label(layout_job.clone());
      }
    }
  }

  fn vertical_empty_separator(&self, ui: &mut Ui, count_spacing: u8) {
    let row_height: f32 = ui.text_style_height(&TextStyle::Body);
    ui.allocate_exact_size(vec2(0.0, row_height * count_spacing as f32), Sense::hover()); // make sure we take up some height
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

  fn add_spacing(&mut self, item_spacing: ItemKind) {
    match self.items.last() {
      Some(ItemKind::ItemSpacing) => {}
      Some(ItemKind::Spacing) => {}
      _ => match item_spacing {
        ItemKind::Spacing => {
          if !self.is_inside_list {
            self.items.push(item_spacing);
          } else {
            self.items.push(ItemKind::ItemSpacing)
          }
        }
        ItemKind::ItemSpacing => {
          self.items.push(item_spacing);
        }
        _ => {}
      },
    }
  }
}
