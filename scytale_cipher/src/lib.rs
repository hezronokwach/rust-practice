
pub fn scytale_cipher(message: String, size: u32) -> String {
    // If the message is empty or size is 0, return the original message
    if message.is_empty() || size == 0 {
        return message;
    }

    let message_len = message.len();
    let size = size as usize;
    
    // Calculate the number of rows needed
    // This is the ceiling of message_len / size
    let rows = (message_len + size - 1) / size;
    
    // Create a result string to hold the cipher
    let mut result = String::with_capacity(message_len);
    
    // Read the message column by column
    for col in 0..size {
        for row in 0..rows {
            let index = row * size + col;
            
            // Only add the character if the index is within the message bounds
            if index < message_len {
                result.push(message.chars().nth(index).unwrap());
            }
        }
    }
    
    result
}