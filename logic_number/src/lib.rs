pub fn narcissistic(n: u64) -> bool {
    // Convert the number to a string to easily get its digits
    let digits_str = n.to_string();
    
    // Get the number of digits
    let num_digits = digits_str.len() as u32;
    
    // Calculate the sum of each digit raised to the power of num_digits
    let sum: u64 = digits_str
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u64)  // Convert char to u64
        .map(|digit| digit.pow(num_digits))       // Raise to power of num_digits
        .sum();                                   // Sum all values
    
    // Check if the sum equals the original number
    sum == n
}