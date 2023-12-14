pub fn score(input: &str) -> u64 {
    let mut total_score: u64 = 0;

    for letter in input.chars() {
        total_score += match letter.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' | 'n' | 'r' | 's' | 't' => 1,
            'd' | 'g' => 2,
            'b' | 'c' | 'm' | 'p' => 3,
            'f' | 'h' | 'v' | 'w' | 'y' => 4,
            'k' => 5,
            'j' | 'x' => 8,
            'q' | 'z' => 10,
            _ => 0, // Handle non-alphabet characters
        };
    }

    total_score
}
