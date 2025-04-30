fn scytale_cipher(message: String, i: u32) -> String {
    let size = i as usize;
    let len = message.len();
    
    // Handle empty message or size 0
    if len == 0 || size == 0 {
        return String::new();
    }
    
    // Calculate number of rows needed
    let rows = (len + size - 1) / size; // Ceiling division
    
    // Create a grid filled with spaces
    let mut grid = vec![vec![' '; size]; rows];
    
    // Fill the grid with message characters
    let chars: Vec<char> = message.chars().collect();
    for idx in 0..len {
        let row = idx / size;
        let col = idx % size;
        grid[row][col] = chars[idx];
    }
    
    // Read the grid column by column
    let mut result = String::new();
    for col in 0..size {
        for row in 0..rows {
            result.push(grid[row][col]);
        }
    }
    
    result.trim_end().to_string()
}