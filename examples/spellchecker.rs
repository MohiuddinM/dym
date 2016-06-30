extern crate dym;

use dym::Lexicon;
use std::fs::File;
use std::io::{stdin, Read};
use std::env::args;

fn main() {
    let file_path = args().nth(1).unwrap();

    let mut lexicon = Lexicon::new();
    let mut file = File::open(file_path).unwrap(); 
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
            println!("    {} is not a word, did you mean:", word);
            for correction in lexicon.corrections_for(&word) {
                println!("        {}", correction);
            }
        }
    }
}
