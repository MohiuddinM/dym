extern crate dym;

use dym::*;
use std::fs::File;
use std::io::{stdin, Read};

const DICT_PATH: &'static str = "/home/taylor/Source/rust/did_you_mean/examples//words.txt";

fn main() {
    let mut lexicon = Lexicon::new();
    let mut file = File::open(DICT_PATH).unwrap(); 
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    for line in contents.lines() {
        lexicon.insert(line);
    }

    println!("All words loaded...");

    loop {
        println!("");
        println!("Type a word!");

        let mut word = String::new();
        stdin().read_line(&mut word).unwrap();
        let word = word.trim();
        if lexicon.contains(&word) {
            println!("    {} is spelled correctly!", word);
        } else {
            println!("    {} is not a valid word! did you mean:", word);
            for suggestion in lexicon.get_suggestions(&word) {
                println!("        {}", suggestion);
            }
        }
    }
}
