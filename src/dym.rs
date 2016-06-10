use std::collections::HashSet;

pub enum Transformation {
    Delete(usize),          // Delete the character at index: usize
    Insert(char, usize),    // Insert the character: char at index: usize
    Swap(usize),            // Swap the character at index: usize with the character at index+1
    Replace(usize, char),   // Replace the character held at index: usize with the character: char
}

pub type Dictionary<'d> = HashSet<&'d str>;

pub trait SpellChecker {
    fn suggest(&self, word: &str) -> Vec<String>;
}

impl<'d> SpellChecker for Dictionary<'d> {

    fn suggest(&self, word: &str) -> Vec<String> {
        let mut suggestions = HashSet::<String>::new();

        // If word is a valid word, don't compute anything
        if let Some(word) = self.get(word) {
            suggestions.insert(word.to_string());
            return suggestions.into_iter().collect();
        }

        for possible in get_permutations(word).into_iter()
                        .filter(|w| self.contains(w.as_str())) {
            suggestions.insert(possible);
        }

        suggestions.into_iter().collect()
    }
}

fn get_permutations(word: &str) -> Vec<String> {
    let mut permutations = Vec::new(); 

    for i in 0..word.len() {
        // Deletions
        permutations.push(delete_char(word, i));

        for c in (b'a'..b'z'+1).map(|c| c as char) {
            // Replacements
            permutations.push(replace_char(word, i, c));
        }
    }

    for i in 0..word.len()-1 {
        // Swaps
        permutations.push(swap_chars(word, i));
    }

    for i in 0..word.len()+1 {
        // Insertions
        for c in (b'a'..b'z'+1).map(|c| c as char) {
            permutations.push(insert_char(word, i, c));
        }
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
    use dym;

    #[test]
    fn permutations() {
        let permutations = dym::get_permutations(&"hello");
        println!("{:?}", permutations);
    }

    #[test]
    fn permutations_for_push() {
        let mut seen_push = false;
        for permutation in dym::get_permutations(&"pus") {
            println!("{}", permutation);
            if permutation == "push".to_string() {
                seen_push = true;
            }
        }
        assert!(seen_push);
    }
    
    #[test]
    fn delete_char() {
        let deleted = dym::delete_char(&"push", 3);
        assert_eq!(deleted, "pus".to_string());
    }
    
    #[test]
    fn swap_chars() {
        let swapped = dym::swap_chars(&"heck", 2);
        assert_eq!(swapped, "hekc".to_string());
    }

    #[test]
    fn replace_char() {
        let replaced = dym::replace_char(&"cow", 1, 'e');
        assert_eq!(replaced, "cew".to_string());
    }

    #[test]
    fn insert_char() {
        let inserted = dym::insert_char(&"cow", 1, 'e');
        assert_eq!(inserted, "ceow".to_string());
    }

    #[test]
    fn insert_char_end() {
        let inserted = dym::insert_char(&"pus", 3, 'h');
        assert_eq!(inserted, "push".to_string());
    }

}
