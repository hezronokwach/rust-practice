pub fn pig_latin(text: &str) -> String {
    // Split the text into words and process each word
    text.split_whitespace()
        .map(transform_word)
        .collect::<Vec<String>>()
        .join(" ")
}

fn transform_word(word: &str) -> String {
    if word.is_empty() {
        return String::new();
    }

    // Check if the word starts with a vowel
    let first_char = word.chars().next().unwrap().to_ascii_lowercase();
    if is_vowel(first_char) {
        return format!("{}ay", word);
    }

    // Get the characters of the word
    let chars: Vec<char> = word.chars().collect();
    
    // Find the index of the first vowel
    let mut first_vowel_idx = 0;
    while first_vowel_idx < chars.len() && !is_vowel(chars[first_vowel_idx]) {
        first_vowel_idx += 1;
    }

    // Handle the special case for "qu"
    if first_vowel_idx > 0 && first_vowel_idx < chars.len() - 1 {
        if chars[first_vowel_idx - 1] == 'q' && chars[first_vowel_idx] == 'u' {
            // Include 'u' in the consonant cluster
            first_vowel_idx += 1;
        }
    }

    // If no vowel was found, move all consonants to the end
    if first_vowel_idx >= chars.len() {
        return format!("{}ay", word);
    }

    let (consonants, rest) = word.split_at(first_vowel_idx);
    format!("{}{}ay", rest, consonants)
}

fn is_vowel(c: char) -> bool {
    matches!(c.to_ascii_lowercase(), 'a' | 'e' | 'i' | 'o' | 'u')
}