use egui::{
  text::LayoutJob, vec2, Align, Color32, FontFamily, FontId, Hyperlink, Label, Layout, RichText,
  ScrollArea, Stroke, TextEdit, TextFormat, TextStyle, Ui,
};
use pulldown_cmark::{Event, HeadingLevel, LinkType, Tag};

use crate::{features::note_preview::NotePreview, ui};

pub struct NotePreviewUi {
  note: NotePreview,
  font_size: f32,
}

struct Link {
  url: String,
  link_type: LinkType,
}

impl Default for Link {
  fn default() -> Self {
    Self {
      url: "".to_string(),
      link_type: LinkType::Inline,
    }
  }
}

enum ItemKind {
  Text,
  Link { url: String, link_type: LinkType },
}

impl Default for ItemKind {
  fn default() -> Self {
    ItemKind::Text
  }
}

#[derive(Default)]
struct Item {
  layout_job: LayoutJob,
  kind: ItemKind,
}

impl Item {
  fn new(layout_job: LayoutJob, kind: ItemKind) -> Self {
    Self { layout_job, kind }
  }
}

impl Default for NotePreviewUi {
  fn default() -> Self {
    Self {
      font_size: 14.0,
      note: NotePreview::default(),
    }
  }
}

impl NotePreviewUi {
  pub fn ui(&mut self, ui: &mut Ui) {
    ui.heading("Note Preview");
    ui.columns(2, |columns| {
      self.render_markdown_input(&mut columns[0]);
      self.render_markdown(&mut columns[1]);
    });
  }

  fn render_markdown_input(&mut self, ui: &mut Ui) {
    ScrollArea::vertical().show(ui, |ui| {
      ui.add(TextEdit::multiline(&mut self.note.markdown_input).desired_width(f32::INFINITY).min_size(vec2(0.0, 300.0)).id_source("source"));
    });
  }

  fn render_markdown(&self, ui: &mut Ui) {
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
    let mut link = Link {
      url: "".to_string(),
      link_type: LinkType::Inline,
    };
    let mut layout_job = LayoutJob::default();

    for event in &mut self.note.parsing_note() {
      match event {
        Event::Start(tag) => match tag {

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
          Tag::Paragraph => {
            current_text_style.font_id.size = self.font_size;
          }
          Tag::CodeBlock(_) => {
            is_code = true;
          }
          Tag::Strong => {
            current_text_style.color = ui.style().visuals.strong_text_color();
          }
          Tag::Emphasis => {
            current_text_style.italics = true;
          }
          Tag::Strikethrough => {
            current_text_style.strikethrough = Stroke::new(1.0, code_text_style.color);
          }
          Tag::Link(link_type, url, _) => {
            link.url = url.to_string();
            link.link_type = link_type.clone();
            is_link = true;
          }
          _ => {
            println!("Start: {:?}", tag)
          }

        },
        Event::End(tag) => match tag {

          Tag::Heading(_, _, _) => {
            current_text_style.font_id.size = self.font_size;
            layout_job.append("\n", 0.0, current_text_style.clone());
          }
          Tag::Paragraph => {
            layout_job.append("\n", 0.0, current_text_style.clone());
          }
          Tag::Strong => {
            current_text_style.color = ui.style().visuals.text_color();
          }
          Tag::Emphasis => {
            current_text_style.italics = false;
          }
          Tag::Strikethrough => {
            current_text_style.strikethrough = Stroke::NONE;
          }
          Tag::CodeBlock(_) => {
            is_code = false;
          }
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
          layout_job.append(
            &s,
            0.0,
            if is_code {
              code_text_style.clone()
            } else {
              current_text_style.clone()
            },
          );
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
          layout_job = LayoutJob::default();
          }
        Event::Code(s) => {
          current_text_style.font_id.family = FontFamily::Monospace;
          current_text_style.background = Color32::from_rgb(0, 0, 0);
          layout_job.append(&s, 0.0, current_text_style.clone());
          items.push(Item::new(layout_job, ItemKind::Text));
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

    let layout = Layout::left_to_right(Align::BOTTOM).with_main_wrap(true);
    let initial_size = vec2(
      ui.available_width(),
      ui.spacing().interact_size.y, // Assume there will be
    );
    ScrollArea::vertical().id_source("renderer").show(ui, |ui| {
      ui.allocate_ui_with_layout(initial_size, layout, |ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        let row_height = ui.text_style_height(&TextStyle::Body);
        ui.set_row_height(row_height);

        for item in items {
          Self::item_render(ui, &item);
        }
      });
    });
  }

  fn item_render(ui: &mut Ui, item: &Item) {
    match &item.kind {
      ItemKind::Text => {
        ui.label(item.layout_job.clone());
      }
      ItemKind::Link { url, .. } => {
        if ui
          .add(Hyperlink::from_label_and_url(item.layout_job.clone(), url))
          .clicked()
        {
          println!("Link clicked to url: {url}");
        };
      }
    }
  }

}
