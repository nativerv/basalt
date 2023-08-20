use egui::{
  text::LayoutJob, Color32, FontFamily, FontId, Label, RichText, Stroke, TextFormat, TextStyle, Ui,
};
use pulldown_cmark::Event;

use crate::features::note_preview::NotePreview;

pub struct NotePreviewUi {
  note: NotePreview,
  font_size: f32,
}

struct Linl {
  is_link: bool,
  url: String,
  text: String}

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
    ui.horizontal(|ui| {
      ui.label("Markdown Input");
      ui.text_edit_multiline(&mut self.note.markdown_input);
    });
    ui.separator();
    ui.heading("Markdown Output");
    self.render_markdown(ui);
  }

  fn render_markdown(&self, ui: &mut Ui) {
    let mut current_text_style = TextFormat::default();
    let code_text_style = {
      let mut default = TextFormat::default();
      default.font_id = FontId::monospace(self.font_size);
      default.background = Color32::from_rgb(0, 0, 0);
      default
    };
    let mut is_code = false;
    let mut is_link = false;
    let mut layout_job = LayoutJob::default();
    for event in &mut self.note.parsing_note() {
      match event {
        Event::Start(tag) => match tag {
          pulldown_cmark::Tag::Heading(level, _, _) => {
            current_text_style.font_id.size = match level {
              pulldown_cmark::HeadingLevel::H1 => self.font_size + 16.0,
              pulldown_cmark::HeadingLevel::H2 => self.font_size + 12.0,
              pulldown_cmark::HeadingLevel::H3 => self.font_size + 8.0,
              pulldown_cmark::HeadingLevel::H4 => self.font_size + 6.0,
              pulldown_cmark::HeadingLevel::H5 => self.font_size + 4.0,
              pulldown_cmark::HeadingLevel::H6 => self.font_size + 2.0,
            };
          }
          pulldown_cmark::Tag::Paragraph => {
            current_text_style.font_id.size = self.font_size;
          }
          pulldown_cmark::Tag::CodeBlock(_) => {
            is_code = true;
          }
          pulldown_cmark::Tag::Strong => {
            current_text_style.color = ui.style().visuals.strong_text_color();
          }
          pulldown_cmark::Tag::Emphasis => {
            current_text_style.italics = true;
          }
          pulldown_cmark::Tag::Strikethrough => {
            current_text_style.strikethrough = Stroke::new(1.0, code_text_style.color);
          }
          pulldown_cmark::Tag::Link(link_type, url, _) => {
            is_link = true;
          }

          _ => {
            println!("Start: {:?}", tag)
          }
        },
        Event::End(tag) => match tag {
          pulldown_cmark::Tag::Heading(_, _, _) => {
            current_text_style.font_id.size = self.font_size;
          }
          pulldown_cmark::Tag::Paragraph => {
            layout_job.append("\n", 0.0, current_text_style.clone());
          }
          pulldown_cmark::Tag::Strong => {
            current_text_style.color = ui.style().visuals.text_color();
          }
          pulldown_cmark::Tag::Emphasis => {
            current_text_style.italics = false;
          }
          pulldown_cmark::Tag::Strikethrough => {
            current_text_style.strikethrough = Stroke::NONE;
          }
          pulldown_cmark::Tag::CodeBlock(_) => {
            is_code = false;
          }
          pulldown_cmark::Tag::Link(link_type, url, _) => {
            is_link = false;
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
        }
        Event::Code(s) => {
          current_text_style.font_id.family = FontFamily::Monospace;
          current_text_style.background = Color32::from_rgb(0, 0, 0);
          layout_job.append(&s, 0.0, current_text_style.clone());
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
    ui.label(layout_job);
  }
}
