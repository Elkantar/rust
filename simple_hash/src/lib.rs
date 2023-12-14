use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut word_count: HashMap<&str, usize> = HashMap::new();

    for word in words {
        *word_count.entry(word).or_insert(0) += 1;
    }

    word_count
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.len()
}
