use std::collections::HashMap;

pub fn word_frequency_counter(words: Vec<&str>) -> HashMap<&str, usize> {
    let mut char_count = HashMap::new();
    for word in words {
        let count = char_count.entry(word).or_insert(0);
        *count += 1;
    }
    char_count
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    let mut count: usize = 0;
    for &value in frequency_count.values() {
        if value == 1 {
            count += 1
        }
    }
    count
}