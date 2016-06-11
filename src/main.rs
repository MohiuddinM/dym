extern crate dym;

use dym::*;
use std::env;

fn main() {
    let mut lexicon = Lexicon::new();
    lexicon.insert("push");
    lexicon.insert("pull");
    lexicon.insert("branch");
    lexicon.insert("stash");
    lexicon.insert("merge");
    lexicon.insert("rebase");
    lexicon.insert("tag");
    lexicon.insert("commit");

    let mut args = env::args();
    let command = args.nth(1).unwrap();

    if !lexicon.contains(&command) {
        println!("Unknown command: '{}' did you mean: ", command);
        for suggestion in lexicon.get_suggestions(&command) {
            println!("    {}", suggestion);
        }
    }
}
