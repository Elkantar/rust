pub fn is_pangram(s: &str) -> bool {
    let mut alphabet = std::collections::HashSet::new();

    for ch in 'a'..='z' {
        alphabet.insert(ch);
    }

    for ch in s.chars() {
        if ch.is_ascii_alphabetic() {
            alphabet.remove(&ch.to_ascii_lowercase());
        }
    }
    alphabet.is_empty()
}