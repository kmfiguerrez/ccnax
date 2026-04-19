use crate::menu;

pub struct Chapter<'a> {
  title: &'a str,
  chapter: &'a str,
  sections: Vec<Section<'a>>
}

impl<'a> Chapter<'a> {
  // Methods.
  pub fn get_title(&self) -> &'a str {
    self.title
  }

  pub fn get_chapter(&self) -> &'a str {
    self.chapter
  }  

  pub fn display_sections(&self) {
    let mut i: u8 = 1;
    for section in &self.sections {
      println!("{}. {}", i, section.get_title());
      i+= 1;
    }
  }

  pub fn get_section(&self, selected_section: u8) -> Option<&Section<'a>> {
    if selected_section == 0 {
      None
    } else {
      self.sections.get((selected_section - 1) as usize)
    }
  }

  // Associate functions.
  pub fn new(title: &'a str, chapter: &'a str, sections: Vec<Section<'a>>) -> Self {
    Self {
      title,
      chapter,
      sections
    }
  }

  pub fn prompt_chapters(chapters: &[Chapter]) {
    // This outer loop will display the chapters and then proceed to selecting a
    // chapter then to section down to headers.
    loop {
      println!("Rust topics\n");

      // Display chapters.
      let mut line_number: u8 = 1;
      for chapter in chapters {
        println!("{}. {}: {}", line_number, chapter.get_chapter(), chapter.get_title());
        line_number += 1;
      }

      println!();

      'prompting_chapter_loop: loop {
        let selected_number = menu::integer_prompt();
        let selected_number = match selected_number {
          Ok(int_input) => {
            // println!("You selected {num}");
            if int_input == 0 {continue;}
            if int_input as usize > chapters.len() {
              continue;
            }
            int_input
          }
          Err(menu::IntegerPromptError::Quit) => {
            println!("Exiting program safely...");
            std::process::exit(0);
          }
          Err(menu::IntegerPromptError::Io(err)) => {
            eprintln!("I/O error: {err}");
            std::process::exit(1);

          }
          Err(menu::IntegerPromptError::Parse(err)) => {
            eprintln!("Parse error: {err}");
            // println!("Select an integer!");
            continue;
          }
        };

        // This part is about to select a chapter.
        // Initialized temporary values to both selected_chapter and selected_section.
        let mut selected_chapter  = &chapters[0];
        for (index, value) in chapters.iter().enumerate() {
          if (selected_number as usize) - 1 == index {
            menu::clear_screen();
            selected_chapter = value;
            break;
          }
        }      
        
        // This part is about to select a section under a selected chapter.
        let mut selected_section: &Section;
        loop {
          // Display chapter title.
          menu::chapter_title(selected_chapter.get_title(), selected_chapter.get_chapter());
          selected_chapter.display_sections();
          println!();
          loop {
            match menu::post_menu_prompt() {
              Ok(menu::PostMenuPromptAction::ListPreviousMenu) => {
                menu::clear_screen();
                break 'prompting_chapter_loop;
              }
              Ok(menu::PostMenuPromptAction::Quit) => std::process::exit(0),
              Ok(menu::PostMenuPromptAction::Integer(int_input)) => {
                // the int_input inside the Integer variant means the selected_section.
                match selected_chapter.get_section(int_input) {
                  Some(section) => {
                    selected_section = section;
                    break;
                  }
                  None => continue
                }
              }
              Err(menu::PostMenuPromptError::InvalidOption(_)) => continue,
              // Err(menu::PostMenuPromptError::Parse(err)) => {
              //   eprintln!("Parse error: {err}");
              //   std::process::exit(1);
              // }
              Err(menu::PostMenuPromptError::Io(err)) => {
                eprintln!("I/O error: {err}");
                std::process::exit(1);
              }
            }
          }

          // This part is about to select a header under a selected section.
          menu::clear_screen();
          selected_section.display_content();
        }
      }
    }
  }
}

pub struct Section<'a> {
  title: &'a str,
  section: &'a str,
  content: fn(title: &str, section: &str)
}

impl<'a> Section<'a> {
  // Methods.
  pub fn get_title(&self) -> &'a str {
    self.title
  }

  pub fn get_section(&self) -> &'a str {
    self.section
  }  

  pub fn display_content(&self) {
    (self.content)(self.get_title(), self.get_section());
  }

  // Associate Functions.
  pub fn new(title: &'a str, section: &'a str, content: fn(&str, &str)) -> Self {
    Self {
      title,
      section,
      content
    }
  }
}


pub struct Header<'a> {
  title: &'a str,
  content: fn()
}

impl<'a> Header<'a> {
  // Methods.
  pub fn get_title(&self) -> &'a str {
    self.title
  }

  pub fn display_content(&self) {
    (self.content)();
  }

  // Associate Functions.
  pub fn new(title: &'a str, content: fn()) -> Self {
    Self {
      title,
      content
    }
  }

  /// Prompt for headers.
  /// 
  /// This associate function list the available headers for a user to select
  /// and display the content of the selected header.
  /// 
  /// # Arguments
  /// 
  /// * `headers` - An Array of Header to display.
  /// * `section_title` - The title of the selected section.
  pub fn prompt_header(headers: &[Header], section_title: &str, section: &str) {
    loop {
      menu::section_title(section_title, section);
      
      let mut i:u8 = 1;
      for header in headers {
        println!("{}. {}",i, header.get_title());
        i+= 1;
      }

      println!();

      'prompting_header_loop: loop {
        let selected_number = menu::post_menu_prompt();
        let selected_number: u8 = match selected_number {
          Ok(menu::PostMenuPromptAction::ListPreviousMenu) => {
            menu::clear_screen();
            return;
          }
          Ok(menu::PostMenuPromptAction::Quit) => std::process::exit(0),
          Ok(menu::PostMenuPromptAction::Integer(int_input)) => {
            if int_input == 0 {continue;}
            if int_input as usize > headers.len() {
              continue;
            }
            int_input
          }
          Err(menu::PostMenuPromptError::InvalidOption(_)) => continue,
          // Err(menu::PostMenuPromptError::Parse(err)) => {
          //   eprintln!("Parse error: {err}");
          //   std::process::exit(1);
          // }
          Err(menu::PostMenuPromptError::Io(err)) => {
            eprintln!("I/O error: {err}");
            std::process::exit(1);
          }        
        };

        // Display selected header content.
        for (index, value) in headers.iter().enumerate() {
          if (selected_number as usize) - 1 == index {
            menu::clear_screen();
            value.display_content();
            break;
          }
        }

        loop {
          match menu::post_header_prompt() {
            Ok(menu::PostHeaderPromptAction::ListPreviousMenu) => {
              menu::clear_screen();
              break 'prompting_header_loop;
            }
            Ok(menu::PostHeaderPromptAction::Quit) => std::process::exit(0),
            Err(menu::PostHeaderPromptError::InvalidOption(_)) => continue,
            Err(menu::PostHeaderPromptError::Io(err)) => {
              eprintln!("I/O error: {err}");
              std::process::exit(1);
            }
          }
        }
      };
    }
  }

}