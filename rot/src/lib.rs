pub fn rotate(input: &str, key: i8) -> String {
    // Convert input to a vector of characters
    input.chars().map(|c| {
        // Only rotate alphabetic characters
        if c.is_ascii_alphabetic() {
            // Determine the base character ('a' for lowercase, 'A' for uppercase)
            let base = if c.is_ascii_lowercase() { 'a' } else { 'A' } as u8;
            
            // Convert to 0-25 range, apply rotation, and wrap around using modulo
            let position = (c as u8) - base;
            let rotated_position = ((position as i16 + key as i16).rem_euclid(26)) as u8;
            
            // Convert back to ASCII and then to char
            (base + rotated_position) as char
        } else {
            // Non-alphabetic characters remain unchanged
            c
        }
    }).collect()
}