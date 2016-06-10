pub mod dym;

#[cfg(test)]
mod tests {

    use dym::*;

    #[test]
    fn integration() {
        let mut dict = Dictionary::new();
        dict.insert("hello");
        dict.insert("retreat");
        dict.insert("potato");
        dict.insert("what");
        dict.insert("when");
        dict.insert("who");
        dict.insert("a");

        let suggestions = dict.suggest(&"helo");
        println!("{:?}", suggestions);
    }
}
