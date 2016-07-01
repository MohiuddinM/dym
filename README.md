# dym
A simple spelling corrector for Rust.

Inspired by [Norvig's Spelling Corrector](http://norvig.com/spell-correct.html)

## Including in Cargo.toml 

    [dependencies]
    dym = "^1.0.0"

## Usage
    
    extern crate dym;

    use dym::Lexicon;

    let mut lex = Lexicon::new();
    lex.insert("hello");
    lex.insert("goodbye");
    lex.insert("hell");

    let corrections = lex.corrections_for("helo");

## Commands Example

Type a misspelled git command.

    cargo run --release --example commands pul

This outputs:

    'pul' is not a command! did you mean:
        push
        pull

## Spellchecker Example 

Takes in a dictionary file and will suggest corrections for words typed through stdin.

    cargo run --release --example spellchecker examples/words.txt

