/*pub fn edit_distance(source: &str, target: &str) -> usize {
    let len1 = source.len();
    let len2 = target.len();

    let mut dp = vec![vec![0; len2 + 1]; len1 + 1];

    for i in 0..=len1 {
        for j in 0..=len2 {
            if i == 0 {
                dp[i][j] = j;
            } else if j == 0 {
                dp[i][j] = i;
            } else if source.as_bytes()[i - 1] == target.as_bytes()[j - 1] {
                dp[i][j] = dp[i - 1][j - 1];
            } else {
                dp[i][j] = 1 + std::cmp::min(
                    dp[i - 1][j],
                    std::cmp::min(
                        dp[i][j - 1],
                        dp[i - 1][j - 1]
                    )
                );
            }
        }
    }

    dp[len1][len2]
}

pub fn expected_variable(input: &str, expected: &str) -> Option<String> {
    // Check if the input string is in camel case or snake case
    if is_camel_case(input) || is_snake_case(input) {
        let similarity = edit_distance(input, expected);

        let max_len = std::cmp::max(input.len(), expected.len());

        // Check if the similarity is more than 50%
        if similarity as f64 / (max_len as f64) < 0.5 {
            let similarity_percentage = (similarity as f64 / max_len as f64 * 100.0).round();
            return Some(format!("{}% close to it", similarity_percentage));
        }
    }
    // Return None if the conditions are not met
    return Some(format!("None"))
}

fn is_camel_case(s: &str) -> bool {
    let mut has_uppercase = false;
    let mut has_underscore = false;

    for c in s.chars() {
        if c.is_uppercase() {
            has_uppercase = true;
        } else if c == '_' {
            has_underscore = true;
        }
    }

    has_uppercase && !has_underscore
}

// Fonction pour vérifier si une chaîne est au format snake_case
fn is_snake_case(s: &str) -> bool {
    let mut has_lowercase = false;
    let mut has_uppercase = false;

    for c in s.chars() {
        if c.is_lowercase() {
            has_lowercase = true;
        } else if c == '_' {
            has_uppercase = true;
        }
    }

    has_lowercase && !has_uppercase
}
*/
use edit_distance::edit_distance;
use strum::IntoEnumIterator;

fn is_camel_case(s: &str) -> bool {
    let words: Vec<&str> = s.split('_').collect();
    words.iter().all(|&w| w.chars().next().map(|c| c.is_uppercase()).unwrap_or(false))
}

fn is_snake_case(s: &str) -> bool {
    let words: Vec<&str> = s.split('_').collect();
    words.iter().all(|&w| w.chars().all(|c| c.is_lowercase()))
}

pub fn expected_variable(compared_str: &str, expected_str: &str) -> Option<String> {
    // Determine the case of the compared string
    let is_camel_case = is_camel_case(compared_str);
    let is_snake_case = is_snake_case(compared_str);

    // Check if it's not in camel case or snake case
    if !is_camel_case && !is_snake_case {
        return Some(format!("None"))
    }

    // Calculate the edit distance
    let distance = edit_distance(compared_str, expected_str);

    // Calculate the threshold for similarity (50%)
    let threshold = (expected_str.len() as f64 * 0.5) as usize;

    // Check if the edit distance is less than or equal to the threshold
    if distance >= threshold {
        return None;
    }

    // Calculate the percentage similarity
    let similarity_percentage = ((expected_str.len() - distance) as f64 / expected_str.len() as f64) * 100.0;

    Some(format!("{:.0}% close to it", similarity_percentage))
}
