pub fn capitalize_first(input: &str) -> String {
    if let Some(first) = input.chars().next() {
        let c = first.to_ascii_uppercase().to_string();
        let r = input
            .chars()
            .skip(1)
            .collect::<String>()
            .to_ascii_lowercase();
        return format!("{}{}", c, r);
    }
    String::new()
}


pub fn title_case(input: &str) -> String {
    // Use a regex to split by whitespace but capture the whitespace
    let mut result = String::new();
    let mut last_end = 0;
    
    // Process each word separately
    for word in input.split_whitespace() {
        // Find the word's position in the original string
        let word_start = input[last_end..].find(word).unwrap() + last_end;
        
        // Add any whitespace that came before this word
        if word_start > last_end {
            result.push_str(&input[last_end..word_start]);
        }
        
        // Add the capitalized word
        result.push_str(&capitalize_first(word));
        
        // Update the last position
        last_end = word_start + word.len();
    }
    
    // Add any trailing whitespace
    if last_end < input.len() {
        result.push_str(&input[last_end..]);
    }
    
    result
}

pub fn change_case(input: &str) -> String {
    let mut result = String::new();
    for c in input.chars() {
        if c.is_uppercase() {
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c.to_uppercase().next().unwrap());
        }
    }
    result
}