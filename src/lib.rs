mod trie;
mod pattern;

use self::trie::Trie;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
pub struct Lexicon {
    words: Trie,
}

impl Lexicon {

    pub fn new() -> Self {
        Lexicon {
            words: Trie::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        self.words.insert(word);
    }

    pub fn insert_all(&mut self, words: &[&str]) {
        for word in words {
            self.insert(word);
        }
    }

    pub fn contains(&self, word: &str) -> bool {
        self.words.contains(word)
    }

    pub fn did_you_mean(&self, word: &str) -> Vec<String> {
        println!("Generating first permutations");
        let perms1 = Lexicon::get_permutations(word); 

        println!("Generating second permutations");
        let mut perms = perms1.clone();
        for perm in perms1.iter() {
            perms.extend_from_slice(Lexicon::get_permutations(perm).as_slice()); 
        }

        println!("Looking up suggestions");
        let mut suggestions = Vec::new();
        for perm in perms.iter() {
            let matches = self.all_matches(perm);
            suggestions.extend_from_slice(matches.as_slice());
        }

        suggestions
    }

    fn get_permutations(word: &str) -> Vec<String> {
        let mut perms = HashSet::new();
        
        // insertions
        for i in 0..word.len() {
            let mut string = word.to_string();
            string.insert(i, '*');
            perms.insert(string);
        }

        // replacements
        for i in 0..word.len() {
            let mut string = word[..i].to_string();
            string.push('*');
            string.push_str(&word[i+1..word.len()]);
            perms.insert(string);
        }
        
        // deletions
        for i in 0..word.len() {
            let mut string = word[..i].to_string();
            string.push_str(&word[i+1..word.len()]);
            perms.insert(string);
        }
        
        // swaps
        for i in 0..word.len()-1 {
            let mut swapped = word[..i].to_string();
            let mut chars = word.chars();
            let first_char = chars.nth(i).unwrap();
            let second_char = chars.next().unwrap();
            swapped.push(second_char);
            swapped.push(first_char);
            swapped.push_str(&word[i+2..word.len()]);
            perms.insert(swapped);
        }
        let result: Vec<String> = perms.into_iter().collect();
        result
    }

    pub fn all_matches(&self, query: &str) -> Vec<String> {
        self.words.all_matches(query)
    }
}

#[cfg(test)]
mod tests {
    use super::Lexicon;
    
    #[test]
    fn test_insert() {
        let mut lex = Lexicon::new();
        lex.insert("hello");
        lex.insert("goodbye");
    }

    #[test]
    fn test_matches() {
        let mut lex = Lexicon::new();
        lex.insert("bog");
        lex.insert("bat");
        lex.insert("sin");
        lex.insert("cat");
        lex.insert("rug");
        lex.insert("cut");
        lex.insert("lay");
        lex.insert("day");
        lex.insert("say");
        lex.insert("cot");

        for m in lex.all_matches("c") {
            println!("{}", m);
        }
    }
}
