pub fn capitalize_first(input: &str) -> String {
    if let Some(first_char) = input.chars().next() {
        let capitalized = first_char.to_uppercase();
        let rest = &input[first_char.len_utf8()..];
        format!("{}{}", capitalized, rest)
    } else {
        String::from(input)
    }
}

pub fn title_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            if let Some(first_char) = word.chars().next() {
                let capitalized = first_char.to_uppercase();
                let rest = &word[first_char.len_utf8()..];
                format!("{}{}", capitalized, rest)
            } else {
                String::from(word)
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn change_case(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else {
                c
            }
        })
        .collect()
}
