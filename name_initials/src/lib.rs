/*pub fn initials(names: Vec<&str>) -> Vec<String> {
    let grouped: Vec<String> = names
        .iter()
        .map(|s| {
            let initials: String = s
                .split_whitespace()
                .map(|word| {
                    if word.chars().all(char::is_alphabetic) {
                        match word.chars().next() {
                            Some(first_char) => {
                                let mut initial = first_char.to_string();
                                if word.len() > 1 {
                                    initial.push('.');
                                }
                                initial
                            }
                            None => String::new(),
                        }
                    } else {
                        word.chars()
                            .filter(|c| c.is_alphabetic())
                            .map(|c| format!("{}.", c))
                            .collect()
                    }
                })
                .collect::<Vec<String>>()
                .join(" ");
            initials
        })
        .collect();

    grouped
}
*/

pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .iter()
        .map(|name| {
            let mut initials = String::new();
            let words: Vec<&str> = name.split_whitespace().collect();
            for (i, word) in words.iter().enumerate() {
                if let Some(first_char) = word.chars().next() {
                    initials.push(first_char);
                    if i < words.len() - 1 {
                        initials.push_str(". ");
                    } else {
                        initials.push_str(".")
                    }
                }
            }
            initials
        })
        .collect()
}

