// Define the error type for cipher validation
#[derive(Debug)]
pub struct CipherError {
    pub expected: String,
}

// The main cipher function that validates if a string is correctly encoded using the Atbash cipher
pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    // Generate the expected Atbash cipher for the original string
    let expected = atbash_encode(original);
    
    // Compare the expected cipher with the provided ciphered string
    if expected == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected })
    }
}

// Helper function to encode a string using the Atbash cipher
fn atbash_encode(input: &str) -> String {
    input
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                // For alphabetic characters, apply the Atbash transformation
                if c.is_ascii_lowercase() {
                    // For lowercase: 'a' becomes 'z', 'b' becomes 'y', etc.
                    // ASCII 'a' is 97, 'z' is 122
                    // The formula is: 'a' + 'z' - c = 97 + 122 - c = 219 - c
                    char::from(219 - c as u8)
                } else {
                    // For uppercase: 'A' becomes 'Z', 'B' becomes 'Y', etc.
                    // ASCII 'A' is 65, 'Z' is 90
                    // The formula is: 'A' + 'Z' - c = 65 + 90 - c = 155 - c
                    char::from(155 - c as u8)
                }
            } else {
                // Non-alphabetic characters remain unchanged
                c
            }
        })
        .collect()
}