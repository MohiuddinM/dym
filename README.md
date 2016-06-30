# dym
A simple spelling corrector for Rust.

Inspired by [Norvig's Spelling Corrector](http://norvig.com/spell-correct.html)

## Usage
    
    use dym::Lexicon;

    let mut lex = Lexicon::new();
    lex.insert("hello");
    lex.insert("goodbye");
    lex.insert("hell");

    let corrections = lex.corrections_for("hel");

## Examples

    cargo run --release --example spellchecker examples/words.txt

    cargo run --release --example commands pul
