pub fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect()
    }
}

pub fn title_case(input: &str) -> String {
    let split = input.split_whitespace();
    for name in split{
        capitalize_first(name);
    }
    input.to_string()
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