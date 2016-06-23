use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq)]
pub struct Lexicon<'a> {
    words: HashSet<String>,
    lookup: Vec<HashMap<char, Vec<&'a String>>>,
}

impl<'a> Lexicon<'a> {

    pub fn new() -> Self {
        Lexicon {
            words: HashSet::new(),
            lookup: Vec::new()
        }
    }

    pub fn insert(&'a mut self, word: &str) {
        self.words.insert(word.to_string());

        for (i, c) in word.chars().enumerate() {
            if self.lookup.get(i) == None {
                let map = HashMap::<char, Vec<&'a String>>::new();
                self.lookup.insert(i, map);
            }
            let map = self.lookup.get_mut(i).unwrap();

            if map.get(&c) == None {
                let list = Vec::<&'a String>::new();
                map.insert(c, list);
            }
            let ptrs = map.get_mut(&c).unwrap();

            ptrs.push(self.words.get(&word.to_string()).unwrap());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Lexicon;
    
    #[test]
    fn test_new() {
        let lex = Lexicon::new();
        assert!(lex.words.len() == 0);
    }

    #[test]
    fn test_insert() {
        let mut lex = Lexicon::new();
        lex.insert("hello");
    }
}
