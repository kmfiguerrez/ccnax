use crate::chapter::{chapter_18::eighteen_point_one, model::Section};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Problem Isolation Using the ping Command", "Section 18.1", eighteen_point_one::content),
  ]
}