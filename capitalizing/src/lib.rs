pub fn capitalize_first(input: &str) -> String {
    if input.is_empty() {
        return String::new();
    }
    
    let first_char = input.chars().next().unwrap().to_uppercase().to_string();
    
    let rest = input.chars().skip(1).collect::<String>();
    
    first_char + &rest
}


pub fn title_case(input: &str) -> String {
    input.split_whitespace()
        .map(capitalize_first)
        .collect::<Vec<String>>()
        .join(" ")
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