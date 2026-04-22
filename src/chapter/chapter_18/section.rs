use crate::chapter::{chapter_18::{eighteen_point_one, eighteen_point_two}, model::Section};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Problem Isolation Using the ping Command", "Section 18.1", eighteen_point_one::content),
    Section::new("Problem Isolation Using the traceroute Command", "Section 18.2", eighteen_point_two::content),
  ]
}