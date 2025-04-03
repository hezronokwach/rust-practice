pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect()
    }
}

pub fn title_case(input: &str) -> String {
    let mut result = String::new();
    let words: Vec<&str> = input.split_whitespace().collect();
    
    for (i, word) in words.iter().enumerate() {
        result.push_str(&capitalize_first(word));
        if i < words.len() - 1 {
            result.push(' ');
        }
    }
    result
}

pub fn change_case(input: &str) -> String {
    let mut result = String::new();
    for chars in input.chars(){
        if chars.is_lowercase(){
            result.push(chars)
        } else if chars.is_uppercase(){
            result.push(chars)
        } else{
            result.push(chars)
        }
    }
    result   
}