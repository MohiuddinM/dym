extern crate dym;

use dym::*;
use std::env;

fn main() {
    let mut dict = Dictionary::new();
    dict.insert("push");
    dict.insert("pull");
    dict.insert("branch");
    dict.insert("stash");
    dict.insert("merge");
    dict.insert("rebase");
    dict.insert("tag");
    dict.insert("commit");

    let mut args = env::args();
    let command = args.nth(1).unwrap();

    if !dict.contains(&*command) {
        println!("Unknown command: '{}' did you mean: ", command);
        for suggestion in dict.suggest(&command) {
            println!("    {}", suggestion);
        }
    }
}
