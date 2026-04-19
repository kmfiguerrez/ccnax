use crate::{chapter::model, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [model::Header; 1];
  subheaders = [
    model::Header::new("Chapter Introduction", ci_content),

  ];

  model::Header::prompt_header(&subheaders, section_title, section);
}


// Subheaders content below.

// Header: Chapter Introduction. Abbreviated as ci.
fn ci_content() {
  let solid_disc = "\u{2022}";

  menu::header_title("Chapter Introduction");
}