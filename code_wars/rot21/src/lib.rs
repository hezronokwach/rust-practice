pub fn rot21(input: &str) -> String {
    input
        .chars() // Iterate over each character in the string
        .map(|c| {
            if c.is_ascii_lowercase() {
                // Shift lowercase letters ('a'..'z')
                (((c as u8 - b'a') + 21) % 26 + b'a') as char
            } else if c.is_ascii_uppercase() {
                // Shift uppercase letters ('A'..'Z')
                (((c as u8 - b'A') + 21) % 26 + b'A') as char
            } else {
                // Leave non-alphabetic characters unchanged
                c
            }
        })
        .collect() // Collect the transformed characters into a new String
}

/*
fn rot21(input: &str) -> String {
    // Define the lowercase and uppercase alphabets
    let lowercase_alphabet = "abcdefghijklmnopqrstuvwxyz";
    let uppercase_alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

    // Create a result string by processing each character
    input
        .chars()
        .map(|c| {
            if let Some(index) = lowercase_alphabet.find(c) {
                // Shift lowercase letters
                lowercase_alphabet.chars().nth((index + 21) % 26).unwrap()
            } else if let Some(index) = uppercase_alphabet.find(c) {
                // Shift uppercase letters
                uppercase_alphabet.chars().nth((index + 21) % 26).unwrap()
            } else {
                // Leave non-alphabetic characters unchanged
                c
            }
        })
        .collect() // Collect the transformed characters into a new String
}
*/