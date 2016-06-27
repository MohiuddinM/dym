use std::slice::Iter;

#[derive(Debug, Eq, PartialEq)]
pub enum CharMatcher {
    Any,
    Exact(char),
}

#[derive(Debug, Eq, PartialEq)]
pub struct Pattern {
    pattern: Vec<CharMatcher>,
    pattern_string: String,
}

impl Pattern {

    pub fn from_str(pattern_str: &str) -> Self {
        let mut pattern = Vec::with_capacity(pattern_str.len());
        for c in pattern_str.chars() {
            let matcher = match c {
                '*' => CharMatcher::Any,
                _ => CharMatcher::Exact(c),
            };
            pattern.push(matcher);
        }

        Pattern {
            pattern: pattern,
            pattern_string: pattern_str.to_string(),
        }
    }

    pub fn iter(&self) -> Iter<CharMatcher> {
        self.pattern.iter()
    }
    
}
