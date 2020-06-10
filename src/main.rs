/*
Program description: Convert text to pig latin. This is when the first consonant
  is moved to the end of the word and appending "ay".
  Example, first -> irst-fay. For words that have a vowel for the first letter
  you append "hay" to the end. For example apple -> appple-hay.

TODO:
- Functions:
  x Display_menu()
  x get_menu_option()
  - get_sentence() -> Sentence
  - get_words() -> Vec<String>
  - convert_sentence(Vec<String>) -> Vec<String>
  - display_sentence(Vec<String>)
*/
use std::io::{self, prelude::*};

enum MenuOption {
  Enter,
  Convert,
  Display,
  Exit,
}

struct Sentence {
  original: String,
  words: Vec<String>,
  converted_words: Vec<String>,
}

impl Sentence {
  fn new() -> Sentence {
    Sentence {
      original: String::new(),
      words: Vec::new(),
      converted_words: Vec::new(),
    }
  }
}

fn main() {
  loop {
    // Display menu
    display_menu();

    // Get menu option
    let user_input = get_menu_option();
    let curr_sentence: Sentence;

    match user_input {
      // Get sentence from user
      MenuOption::Enter => curr_sentence = get_sentence(),
      // Convert words to pig latin
      MenuOption::Convert => continue,
      // Display converted sentence
      MenuOption::Display => continue,
      // Display new sentence
      MenuOption::Exit => {
        println!("\nExiting...");
        break;
      }
    }
  }
}

fn display_menu() {
  println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
  println!("Welcome to pig latin converter");
  println!("1) Enter sentence");
  println!("2) Convert sentence");
  println!("3) Display converted sentence");
  println!("4) Exit");
  print!("> ");

  io::stdout().flush().ok().expect("Could not flush stdout");
}

fn get_menu_option() -> MenuOption {
  let user_input: MenuOption = loop {
    let mut user_input = String::new();

    io::stdin()
      .read_line(&mut user_input)
      .expect("Failed to read input");
    let user_input: MenuOption = match user_input.trim().parse() {
      Ok(num) => match num {
        1 => MenuOption::Enter,
        2 => MenuOption::Convert,
        3 => MenuOption::Display,
        4 => MenuOption::Exit,
        _ => {
          println!("Error: invalid menu option");
          println!("Please enter a valid menu option");

          continue;
        }
      },
      Err(_) => {
        println!("Error: invalid menu option");
        println!("Please enter a valid menu option");

        continue;
      }
    };

    break user_input;
  };

  user_input
}

fn get_sentence() -> Sentence {
  let user_input = loop {
    let mut user_input = String::new();

    io::stdin()
      .read_line(&mut user_input)
      .expect("Failed to read line");

    let user_input = user_input.trim().to_string();

    if user_input == "" {
      println!("Input a sentence to convert");
      continue;
    }

    break user_input;
  };

  let mut curr_sentence = Sentence::new();
  curr_sentence.original = user_input.to_string();
  curr_sentence
}
