extern crate dym;

use dym::Lexicon;
use std::env::args;

fn main() {
    let mut lexicon = Lexicon::new();
    lexicon.insert_all(&vec![
        "config", "help", "init", "clone", "add", "status", "diff",
        "commit", "reset", "rm", "mv", "branch", "checkout", "merge",
        "mergetool", "log", "stash", "tag", "fetch", "pull", "push",
        "remote", "submodule", "show", "log", "diff", "shortlog",
        "describe", "apply", "cherry-pick", "diff", "rebase", "revert",
        "bisect", "blame", "grep", "clean", "gc","fsck", "reflog",
        "filter-branch", "instaweb", "archive", "bundle"
    ]);
    
    let mut args = args(); 
    let command = args.nth(1).unwrap_or_else(|| {
        panic!("Type a git command as the first argument!"); 
    });

    if lexicon.contains(&command) {
        println!("Doing {}!", command);
    } else {
        let corrections = lexicon.corrections_for(&command);
        println!("'{}' is not a command! did you mean:", command);
        for correction in corrections.into_iter() {
            println!("    {}", correction);
        }
    }
}
