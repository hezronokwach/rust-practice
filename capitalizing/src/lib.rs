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