pub fn first_subword(mut s: String) -> String {
    let mut result = String::new();
    let mut first_char = true;

    for c in s.chars() {
        if first_char || c.is_alphanumeric() && !c.is_uppercase() {
            result.push(c);
            first_char = false;
        } else {
            break;
        }
    }

    result
}

