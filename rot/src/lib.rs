pub fn rotate(input: &str, key: i8) -> String {
    input
        .chars()
        .map(|c| match c {
            'a'..='z' => rotate_char(c, key, 'a' as u8, 'z' as u8),
            'A'..='Z' => rotate_char(c, key, 'A' as u8, 'Z' as u8),
            _ => c,
        })
        .collect()
}

fn rotate_char(c: char, key: i8, start: u8, end: u8) -> char {
    let shifted = (c as u8) - start;
    let rotated = (shifted as i8 + key) % 26;
    let result = start + (rotated + 26) as u8 % 26;
    result as char
}
