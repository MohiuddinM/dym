use std::collections::{HashMap, HashSet};
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
pub struct Lexicon {
    words: HashSet<Rc<String>>,
    lookup: HashMap<usize, Vec<HashMap<char, Vec<Rc<String>>>>>,
    lookup_cache: HashMap<String, Vec<Rc<String>>>,
}

impl Lexicon {

    pub fn new() -> Self {
        Lexicon {
            words: HashSet::new(),
            lookup: HashMap::new(),
            lookup_cache: HashMap::new(),
        }
    }

    pub fn insert(&mut self, word: &str) {
        let rc = Rc::new(word.to_string());
        self.words.insert(rc.clone());
        
        let len_idx = word.len();
        if self.lookup.get(&len_idx) == None {
            let list = Vec::new();
            self.lookup.insert(len_idx, list);
        }
        let len = self.lookup.get_mut(&len_idx).unwrap();

        for (i, c) in word.chars().enumerate() {
            if len.get(i) == None {
                let map = HashMap::<char, Vec<Rc<String>>>::new();
                len.insert(i, map);
            }
            let map = len.get_mut(i).unwrap();

            if map.get(&c) == None {
                let list = Vec::<Rc<String>>::new();
                map.insert(c, list);
            }
            let ptrs = map.get_mut(&c).unwrap();

            ptrs.push(rc.clone());
        }
    }

    pub fn did_you_mean(&self, word: &str) -> Vec<String> {
        println!("Generating first permutations");
        let perms1 = Lexicon::get_permutations(word); 

        println!("Generating second permutations");
        let mut perms = perms1.clone();
        for perm in perms1.iter() {
            perms.extend_from_slice(Lexicon::get_permutations(perm).as_slice()); 
        }

        println!("{:?}", perms);
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
        let mut matches: Vec<HashSet<Rc<String>>> = Vec::new();
        let len = self.lookup.get(&query.len());
        if len == None {
            return Vec::new();
        }
        let len = len.unwrap();

        for (i, c) in query.chars().enumerate() {
            match c {
                '*' => {},
                _ => {
                    let map = len.get(i);
                    if map == None {
                        return Vec::new();
                    }
                    let map = map.unwrap();

                    let words = map.get(&c);
                    if words == None {
                        return Vec::new();
                    }
                    let words = words.unwrap();

                    let wordset = words.iter().cloned().collect();
                    matches.push(wordset); 
                }
            }
        }

        let mut intersection = self.words.clone();
        for set in matches.into_iter() {
            let sub_int: HashSet<Rc<String>> = {intersection}.intersection(&set).cloned().collect();
            intersection = sub_int;
        }
        let result: Vec<String> = intersection.iter().map(|s| s.to_string()).collect();
        result
    }

    fn matches(word1: &str, word2: &str) -> bool {
        if word1.len() != word2.len() {
            return false; 
        }

        for (i, c1) in word1.char_indices() {
            let c2 = word2.chars().nth(i);
            if c2 == None {
                return false;
            }
            let c2 = c2.unwrap();
            let wildcard = c1 == '*' || c2 == '*';
            if c1 != c2 && !wildcard {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::Lexicon;
    
    #[test]
    fn test_wildcard() {
        let matches = Lexicon::matches("word", "w*r*");
        assert!(matches);

        let matches = Lexicon::matches("word", "words");
        assert!(!matches);
    }

    #[test]
    fn test_new() {
        let lex = Lexicon::new();
        assert!(lex.words.len() == 0);
    }

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
