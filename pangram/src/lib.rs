pub fn is_pangram(sentence: &str) -> bool {
    // If the sentence is empty, it can't be a pangram
    if sentence.is_empty() {
        return false;
    }

    // Create a set of all lowercase letters in the sentence
    let letters: std::collections::HashSet<char> = sentence
        .to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();

    // Check if we have all 26 letters of the English alphabet
    letters.len() == 26
}