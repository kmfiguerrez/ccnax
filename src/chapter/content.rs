use crate::chapter::{chapter_18, model};


pub fn generate_chapters() -> [model::Chapter<'static>; 1] {
  let chapters: [model::Chapter<'_>; 1] = [
    model::Chapter::new(
      "Troubleshooting IPv4 Routing",
      "Chapter 18",
      chapter_18::section::generate_sections()
    )
  ];

  chapters
}