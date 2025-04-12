pub fn num_to_ordinal(number: u32) -> String {
    let suffix = match (number % 100, number % 10) {
        (11..=13, _) => "th", // 11th, 12th, 13th are special cases
        (_, 1) => "st",       // numbers ending in 1 (except 11) get "st"
        (_, 2) => "nd",       // numbers ending in 2 (except 12) get "nd"
        (_, 3) => "rd",       // numbers ending in 3 (except 13) get "rd"
        _ => "th",            // all other numbers get "th"
    };
    
    format!("{}{}", number, suffix)
}