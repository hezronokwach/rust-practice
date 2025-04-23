pub fn to_camel_case(text: &str) -> String {
    let mut result = String::new();
    let words: Vec<&str> = text.split(|c: char| c == '-' || c == '_').collect();

    for (i, word) in words.iter().enumerate() {
        if word.is_empty() {
            continue; 
        }
        if i == 0 {
            result.push_str(&word);
        } else {
            let mut chars = word.chars();
            if let Some(first_char) = chars.next() {
                result.push(first_char.to_ascii_uppercase());
                result.push_str(&chars.as_str().to_ascii_lowercase());
            }
        }
    }

    result
}