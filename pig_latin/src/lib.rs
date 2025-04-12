pub fn pig_latin(text: &str) -> String {
    text.split_whitespace()
        .map(|word| {
            if word.is_empty() {
                return String::new();
            }

            let first_char = word.chars().next().unwrap().to_ascii_lowercase();
            
            // Rule 1: If word begins with a vowel, just add "ay" to the end
            if is_vowel(first_char) {
                return format!("{}ay", word);
            }
            
            // Check for words starting with "qu"
            if word.len() >= 2 {
                let first_two: String = word.chars().take(2).collect();
                if first_two.to_lowercase() == "qu" {
                    let rest = &word[2..];
                    return format!("{}{}ay", rest, &word[..2]);
                }
            }
            
            // For other words starting with consonants
            let mut i = 0;
            let chars: Vec<char> = word.chars().collect();
            
            // Find the first vowel
            while i < chars.len() && !is_vowel(chars[i].to_ascii_lowercase()) {
                i += 1;
            }
            
            // If no vowel was found, move all consonants to the end
            if i >= chars.len() {
                return format!("{}ay", word);
            }
            
            // Check for "qu" after initial consonants
            if i > 0 && i < chars.len() - 1 {
                if chars[i-1].to_ascii_lowercase() == 'q' && chars[i].to_ascii_lowercase() == 'u' {
                    // Skip the 'u' as well
                    i += 1;
                }
            }
            
            // Split the word and rearrange
            format!("{}{}ay", &word[i..], &word[..i])
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn is_vowel(c: char) -> bool {
    matches!(c, 'a' | 'e' | 'i' | 'o' | 'u')
}