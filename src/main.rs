/*
Program description: Convert text to pig latin. This is when the first consonant
  is moved to the end of the word and appending "ay".
  Example, first -> irst-fay. For words that have a vowel for the first letter
  you append "hay" to the end. For example apple -> appple-hay.
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
    let mut curr_sentence: Sentence = Sentence::new();
    loop {
        // Display menu
        display_menu();

        // Get menu option
        let user_input = get_menu_option();

        match user_input {
            // Get sentence from user
            MenuOption::Enter => curr_sentence = get_sentence(),
            // Convert words to pig latin
            MenuOption::Convert => {
                curr_sentence.converted_words = convert_words(&curr_sentence.words)
            }
            // Display converted sentence
            MenuOption::Display => display_sentence(&curr_sentence),
            // Display new sentence
            MenuOption::Exit => {
                println!("\nExiting...");
                break;
            }
        }
    }
}

fn display_menu() {
    println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
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

    let mut sentence = Sentence::new();
    sentence.original = user_input.to_string();
    sentence.words = get_words(user_input.to_string());
    sentence
}

fn get_words(sentence: String) -> Vec<String> {
    let mut words = Vec::new();

    for word in sentence.split(" ") {
        let mut punctuation_index: usize = 0;
        for (index, character) in word.chars().enumerate() {
            let character = character as u32;
            if (character > 96 && character < 123)
                || (character > 64 && character < 91)
                || (character == 44 || character == 39)
            {
                continue;
            } else {
                punctuation_index = index;
            }
        }
        println!("word: {}, punctuation_index: {}", word, punctuation_index);

        if punctuation_index > 0 {
            words.push(word[0..punctuation_index].to_string());
        } else {
            words.push(word.to_string());
        }
    }

    words
}

fn convert_words(words: &Vec<String>) -> Vec<String> {
    let mut converted_words = Vec::new();
    for word in words.iter() {
        let mut converted_word = String::new();
        let mut first_letter: char = 'z';
        for (index, character) in word.chars().enumerate() {
            if index == 0 {
                if is_vowel(character) {
                    converted_word.push(character);
                }
                first_letter = character;
            } else if index == word.len() - 1 {
                converted_word.push(character);

                if is_vowel(first_letter) {
                    converted_word.push_str(&format!("-hay"));
                } else {
                    converted_word.push_str(&format!("-{}ay", first_letter));
                }
            } else {
                converted_word.push(character);
            }
        }

        converted_words.push(converted_word);
    }

    converted_words
}

fn is_vowel(letter: char) -> bool {
    match letter {
        'a' => true,
        'e' => true,
        'i' => true,
        'o' => true,
        'u' => true,
        _ => false,
    }
}

fn display_sentence(sentence: &Sentence) {
    for word in sentence.converted_words.iter() {
        print!("{} ", word);
    }
}
