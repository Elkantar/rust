/*pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    words.sort_by_key(|word| {
        let position: usize = word
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap();
        position
    });

    let sorted_phrase = words.join(" ");

    let result = remove_digits(&sorted_phrase);

    result
}

pub fn remove_digits(input: &str) -> String {
    let result: String = input.chars().filter(|c| !c.is_digit(10)).collect();
    result
}
*/

pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split_whitespace().collect();

    words.sort_by_key(|word| {
        let position: usize = word
            .chars()
            .filter_map(|c| c.to_digit(10))
            .fold(0, |acc, digit| acc * 10 + digit as usize);
        position
    });

    let sorted_phrase = words.join(" ");

    let result = remove_digits(&sorted_phrase);

    result
}

pub fn remove_digits(input: &str) -> String {
    let result: String = input.chars().filter(|c| !c.is_digit(10)).collect();
    result
}
