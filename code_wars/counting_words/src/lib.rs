use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
    let mut result = HashMap::new();
    let cleaned: String = words
        .chars()
        .filter(|c| c.is_ascii_lowercase() || c.is_ascii_uppercase() || c.is_whitespace())
        .collect();
    
    let result2 = splitt(&cleaned);
    
    for splits in result2.iter() {
        *result.entry(splits.to_string()).or_insert(0) += 1;
    }
    
    result
}

fn splitt(words: &str) -> Vec<String> {
    words
        .to_lowercase()
        .split(|c: char| c == ' ' || c == '\t' || c == '\n')
        .filter(|s| !s.is_empty() && s.chars().all(|c| c.is_ascii_lowercase()))
        .map(|s| s.to_string())
        .collect()
}