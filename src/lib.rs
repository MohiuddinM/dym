extern crate fnv;
extern crate strsim;


mod trie;
mod pattern;

use self::trie::Trie;
use std::collections::HashSet;
//use fnv::FnvHashMap;
use fnv::FnvHashSet;
use fnv::FnvBuildHasher;

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

    //pub fn new_from_file(path: &str) {  
    //}

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

    pub fn best_match_for(&self, word: &str) -> String {
        let corrections = self.corrections_for(&word);
        let (_, correct_word) = corrections.iter().map(|test_word| (strsim::levenshtein(test_word, &word), test_word))
        .fold((usize::max_value(), ""), |(ar, aw), (br, bw)| if br < ar { (br, bw) } else { (ar, aw) });
        correct_word.into()
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

        //let mut corrections = HashSet::new();
        let mut corrections: HashSet<String, FnvBuildHasher> = FnvHashSet::default();
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
        //let mut perms = HashSet::new();
        let mut perms: HashSet<String, FnvBuildHasher> = FnvHashSet::default();
        
        // insertions
        for i in 0..word.len() {
            let mut string: String = word.into();
            string.insert(i, '*');
            perms.insert(string);
        }

        // replacements
        for i in 0..word.len() {
            let mut string: String = word[..i].into();
            string.push('*');
            string.push_str(&word[i+1..word.len()]);
            perms.insert(string);
        }
        
        // deletions
        for i in 0..word.len() {
            let mut string: String = word[..i].into();
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
