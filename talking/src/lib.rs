/*pub fn talking(text: &str) -> &str {
    if text.is_empty() {
        "Just say something!"
    } else if text == text.to_uppercase() {
        if text.ends_with('?') {
            "Quiet, I am thinking!"
        } else {
            "There is no need to yell, calm down!"
        }
    } else if text.ends_with('?') {
        "Sure."
    } else {
        "Interesting"
    }
}
*/
pub fn talking(text: &str) -> &'static str {
    let has_letters = text.chars().any(|c| c.is_alphabetic());
    let is_yelling = has_letters
        && text.chars().all(|c| c.is_uppercase() || !c.is_alphabetic())
        && !text.chars().all(|c| c.is_numeric());

    let is_question = text.trim().ends_with('?');

    if text.trim().is_empty() {
        "Just say something!"
    } else if is_yelling && is_question {
        "Quiet, I am thinking!"
    } else if is_yelling {
        "There is no need to yell, calm down!"
    } else if is_question {
        "Sure."
    } else {
        "Interesting"
    }
}