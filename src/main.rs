mod common;
mod trie;
use crate::common::{Ascii, WORDS};
use crate::trie::Trie;
use std::io;

fn main() {
    let mut trie = Trie::new();
    for word in WORDS {
        trie.insert(word);
    }

    println!("\nYou are ready to go!\n");
    
    loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read user input");
        let input = match input.parse::<String>() {
            Ok(input) => input,
            Err(_) => String::from("Failed to parse user input as String."),
        };

        match input.is_ascii() {
            true => {
                print!("Words that begin with: {}", input);
                let matching_words: Vec<Ascii> = trie.search_all(&input.trim());
                if matching_words.len() > 0 {
                    for word in matching_words {
                        println!("{}", word.to_string())
                    }
                } else {
                    println!("None")
                }
                print!("\n");
            }
            _ => {
                println!("Only letters from english alphabet are allowed.")
            }
        };
    }
}
