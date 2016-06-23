pub mod evaluator;

use std::collections::{HashMap, HashSet};
use std::thread;

pub struct Lexicon {
    words: HashSet<String>,
    chars: HashMap<usize, HashSet<char>>,
}

impl Lexicon {
    pub fn new() -> Self {
        Lexicon { 
            words: HashSet::new(),
            chars: HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, word: &str) {
        let word = word.to_lowercase().trim().to_string();
        self.words.insert(word.clone());
        for (i, c) in word.chars().enumerate() {
            if !self.chars.contains_key(&i) {
                let mut set = HashSet::new();
                set.insert(c);
                self.chars.insert(i, set);
            } else {
                self.chars.get_mut(&i).unwrap().insert(c);
            }
        }
    }
    
    pub fn insert_all(&mut self, words: Vec<&str>) {
        for word in words.into_iter() {
            self.insert(word);
        }
    }

    pub fn contains(&self, word: &str) -> bool {
        self.words.contains(&word.to_lowercase().trim().to_string())
    }

    pub fn get_suggestions(&self, word: &str) -> Vec<String> {
        let mut suggestions = HashSet::<String>::new();

        // If word is a valid word, don't compute anything
        if self.contains(word.to_lowercase().trim()) {
            suggestions.insert(word.to_string());
            return suggestions.into_iter().collect();
        }

        let perm1 = get_permutations(&self.chars, word.to_lowercase().trim());
        let single_edits: Vec<String> = perm1.iter().filter(|w| self.words.contains(*w)).map(|w| w.clone()).collect();
        if !single_edits.is_empty() {
            return single_edits;
        }

        let mut handles = Vec::new();
        for permutation in perm1.iter() {
            let permutation = permutation.to_string();
            let chars = self.chars.clone();
            handles.push(thread::spawn(move || {
                get_permutations(&chars, &permutation) 
            }));
        }
        
        let mut perm2 = Vec::new();
        for handle in handles.into_iter() {
            perm2.extend_from_slice(handle.join().unwrap().as_slice());
        }

        for possible in perm1.into_iter()
                             .chain(perm2.into_iter())
                             .filter(|w| self.words.contains(w)) {
            suggestions.insert(possible);
        }
                                     
        suggestions.into_iter().collect()
    }
}

fn get_permutations(chars: &HashMap<usize, HashSet<char>>, word: &str) -> Vec<String> {
    let mut handles = Vec::new();
    // Deletions
    let del_word = word.clone().to_string();
    handles.push(thread::spawn(move || {
        let mut deletions = Vec::new();
        for i in 0..del_word.len() {
            deletions.push(delete_char(&del_word, i));
        }
        deletions
    }));

    // Replacements
    let rep_word = word.clone().to_string();
    let rep_chars = chars.clone();
    handles.push(thread::spawn(move || {
        let mut replacements = Vec::new();
        for i in 0..rep_word.len() {
            if let Some(set) = rep_chars.get(&i) {
                for c in set.iter(){
                    replacements.push(replace_char(&rep_word, i, *c));
                }
            }
        }
        replacements
    }));

    // Swaps
    let swap_word = word.clone().to_string();
    handles.push(thread::spawn(move || {
        let mut swaps = Vec::new();
        for i in 0..swap_word.len()-1 {
            swaps.push(swap_chars(&swap_word, i));
        }
        swaps
    }));

    // Insertions
    let ins_word = word.clone().to_string();
    let ins_chars = chars.clone();
    handles.push(thread::spawn(move || {
        let mut inserts = Vec::new();
        for i in 0..ins_word.len()+1 {
            if let Some(set) = ins_chars.get(&i) {
                for c in set.iter() {
                    inserts.push(insert_char(&ins_word, i, *c));
                }
            }
        }
        inserts
    }));

    let mut permutations = Vec::<String>::new();
    for handle in handles.into_iter() {
        permutations.extend_from_slice(handle.join().unwrap().as_slice());
    }
    permutations
}

fn delete_char(word: &str, i: usize) -> String {
    let mut deletion = word[..i].to_string();
    deletion.push_str(&word[i+1..word.len()]);
    deletion
}

fn swap_chars(word: &str, i: usize) -> String {
    let mut swapped = word[..i].to_string();
    let mut chars = word.chars();
    let first_char = chars.nth(i).unwrap();
    let second_char = chars.next().unwrap();
    swapped.push(second_char);
    swapped.push(first_char);
    swapped.push_str(&word[i+2..word.len()]);
    swapped
}

fn replace_char(word: &str, i: usize, replacement: char) -> String {
    let mut replaced = word[..i].to_string();
    replaced.push(replacement);
    replaced.push_str(&word[i+1..word.len()]);
    replaced
}

fn insert_char(word: &str, i: usize, insert: char) -> String {
    let mut inserted = word[..i].to_string();
    inserted.push(insert);
    inserted.push_str(&word[i..word.len()]);
    inserted
}

#[cfg(test)]
mod tests {
    use super::{get_permutations, delete_char, replace_char, insert_char, swap_chars};

    #[test]
    fn permutations_test() {
        let permutations = get_permutations(&"hello");
        println!("{:?}", permutations);
    }

    #[test]
    fn permutations_for_push_test() {
        let mut seen_push = false;
        for permutation in get_permutations(&"pus") {
            println!("{}", permutation);
            if permutation == "push".to_string() {
                seen_push = true;
            }
        }
        assert!(seen_push);
    }
    
    #[test]
    fn delete_char_test() {
        let deleted = delete_char(&"push", 3);
        assert_eq!(deleted, "pus".to_string());
    }
    
    #[test]
    fn swap_chars_test() {
        let swapped = swap_chars(&"heck", 2);
        assert_eq!(swapped, "hekc".to_string());
    }

    #[test]
    fn replace_char_test() {
        let replaced = replace_char(&"cow", 1, 'e');
        assert_eq!(replaced, "cew".to_string());
    }

    #[test]
    fn insert_char_test() {
        let inserted = insert_char(&"cow", 1, 'e');
        assert_eq!(inserted, "ceow".to_string());
    }

    #[test]
    fn insert_char_end_test() {
        let inserted = insert_char(&"pus", 3, 'h');
        assert_eq!(inserted, "push".to_string());
    }

}
