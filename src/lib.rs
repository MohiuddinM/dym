mod trie;
mod pattern;

use self::trie::Trie;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq)]
pub struct Lexicon {
    words: Trie,
}

impl Lexicon {

    /// Creates an empty Lexicon
    pub fn new() -> Self {
        Lexicon {
            words: Trie::new(),
        }
    }

    /// Inserts a copy of the given word into the Lexicon
    /// after converting it to lowercase and trimming whitespace
    pub fn insert(&mut self, word: &str) {
        let word = word.trim().to_lowercase(); 
        self.words.insert(&word);
    }

    /// Inserts a copy of all given words into the Lexicon
    /// after converting them to lowercase and trimming whitespace
    pub fn insert_all(&mut self, words: &[&str]) {
        for word in words {
            self.insert(word);
        }
    }

    /// Returns true if the Lexicon contains the given word
    pub fn contains(&self, word: &str) -> bool {
        let word = word.trim().to_lowercase();
        self.words.contains(&word)
    }

    /// Returns all words in the Lexicon that are at most
    /// two edits away from the given word.
    pub fn corrections_for(&self, word: &str) -> Vec<String> {
        let word = word.trim().to_lowercase();

        let perms1 = Lexicon::generate_permutations(&word); 
        let mut perms = perms1.clone();
        for perm in perms1.iter() {
            perms.extend_from_slice(Lexicon::generate_permutations(perm).as_slice()); 
        }

        let mut corrections = HashSet::new();
        for perm in perms.iter() {
            let matches = self.words.all_matches(perm);
            for m in matches.into_iter() {
                corrections.insert(m);
            }
        }

        corrections.into_iter().collect() 
    }

    /// Generates all words that are 1 edit away from the given word.
    /// Wildcards are used to minimize the number of permutations generated.
    fn generate_permutations(word: &str) -> Vec<String> {
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
        
        let result: Vec<String> = perms.into_iter().collect();
        result
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
}
