use std::io::{self, Write};
use owo_colors::OwoColorize;
use thiserror::Error;
use std::process::Command;


pub fn integer_prompt() -> Result<u8, IntegerPromptError> {
  print!("Select a number (q to quit): ");
  io::stdout().flush().expect("failed to flush stdout");

  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(0) => {
      // EOF (Ctrl+D/Ctrl+Z) — exit
      // println!("Read 0 bytes");
    }
    Ok(_) => {
      let input = input.trim();
      if input == "q" {
        // println!("Quitting.");
        return Err(IntegerPromptError::Quit);
      }
      // println!("You entered: {}", input);
      // println!("{n} bytes read");
    }
    Err(err) => {
      // eprintln!("Error reading input: {}", err);
      return Err(IntegerPromptError::Io(err));
    }
  }

  // Parse user input into integers.
  let input: u8 = match input.trim().parse() {
    Ok(num) => num,
    Err(err) => {
      // eprintln!("Error reading input: {}", err);
      // println!("Program terminated! You did not enter an integer!");
      return Err(IntegerPromptError::Parse(err));
    }
  };
  Ok(input)
}


// This code uses the thiserror library.
#[derive(Error, Debug)]
pub enum IntegerPromptError {
    #[error("User chose to quit")]
    Quit, // Our new variant

    #[error("IO error: {0}")]
    Io(#[from] io::Error), // Handles flush and read_line

    #[error("Parsing error: {0}")]
    Parse(#[from] std::num::ParseIntError), // Handles parse
}

pub fn clear_screen() {
  // Clear previous screen.
  if cfg!(target_os = "windows") {
    Command::new("cmd").args(["/c", "cls"]).status().unwrap();
  } 
  else {
    Command::new("clear").status().unwrap();
  }       
}


// Action returned when reading input that can be an integer, quit, or list previous menu.
#[derive(Debug)]
pub enum PostMenuPromptAction {
    Integer(u8),
    Quit,
    ListPreviousMenu,
}

// Error type for the menu input reader. Uses `thiserror` for nice messages.
#[derive(Error, Debug)]
pub enum PostMenuPromptError {
    #[error("invalid option: {0}")]
    InvalidOption(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    // #[error("Parsing error: {0}")]
    // Parse(#[from] std::num::ParseIntError),
}

/// Reads user input that accepts integers, 'q' for quit, or 'c' to list previous menu.
/// Returns a `PostMenuPromptAction` on success.
pub fn post_menu_prompt() -> Result<PostMenuPromptAction, PostMenuPromptError> {
  print!("Enter an integer, 'q' to quit, or 'p' to list previous menu: ");
  io::stdout().flush()?;

  let mut input = String::new();
  let n = io::stdin().read_line(&mut input)?;
  if n == 0 {
      return Err(PostMenuPromptError::InvalidOption("EOF".into()));
  }

  let input = input.trim();
  match input {
      "q" | "Q" => Ok(PostMenuPromptAction::Quit),
      "p" | "P" => Ok(PostMenuPromptAction::ListPreviousMenu),
      other => {
          match other.parse::<u8>() {
              Ok(num) => Ok(PostMenuPromptAction::Integer(num)),
              Err(_) => Err(PostMenuPromptError::InvalidOption(other.to_string())),
          }
      }
  }
}


// Action returned when reading the to quit or display previous menu input.
pub enum PostHeaderPromptAction {
  Quit,
  ListPreviousMenu,
}

// Error type for the small menu input reader. Uses `thiserror` for nice messages.
#[derive(Error, Debug)]
pub enum PostHeaderPromptError {
  #[error("invalid option: {0}")]
  InvalidOption(String),

  #[error("IO error: {0}")]
  Io(#[from] io::Error),
}

/// Reads a single menu command from the user and returns a `PostHeaderPromptAction`.
/// Only accepts `q` (quit) or `s` (list subheaders). Any other input is an error.
pub fn post_header_prompt() -> Result<PostHeaderPromptAction, PostHeaderPromptError> {
  print!("Enter 'q' to quit or 's' to list subheaders: ");
  io::stdout().flush()?;

  let mut input = String::new();
  let n = io::stdin().read_line(&mut input)?;
  if n == 0 {
    return Err(PostHeaderPromptError::InvalidOption("EOF".into()));
  }

  match input.trim() {
    "q" | "Q" => Ok(PostHeaderPromptAction::Quit),
    "s" | "S" => Ok(PostHeaderPromptAction::ListPreviousMenu),
    other => Err(PostHeaderPromptError::InvalidOption(other.to_string())),
  }
}

/// This function will format a literal string into a style desired for a title.
pub fn chapter_title(title: &str, chapter: &str) {
  let chapter_title = format!("{}: {}", chapter, title);
  println!("{}", chapter_title.bright_blue().bold());
  println!("{} \n", "-".repeat(chapter_title.len()).bright_blue());
}

/// This function will format a literal string into a style desired for a title.
pub fn section_title(title: &str, section: &str) {
  let section_title = format!("{}: {}", section, title);
  println!("{}", section_title.bright_blue().bold());
  println!("{} \n", "-".repeat(section_title.len()).bright_blue());  
}

/// This function will format a literal string into a style desired for a title.
pub fn subheader_title(title: &str) {
  println!("{}", title.bright_blue().bold());
  println!("{} \n", "-".repeat(title.len()).bright_blue());  
}
