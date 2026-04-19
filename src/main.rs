use ccnax::chapter::{content::generate_chapters, model};

fn main() {
  let chapters = generate_chapters();
  
  model::Chapter::prompt_chapters(&chapters);
}
