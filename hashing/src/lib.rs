pub fn mean(list: &[i32]) -> f64 {
    if list.is_empty() {
        return 0.0;
    }
    
    let sum: i32 = list.iter().sum();
    sum as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    if list.is_empty() {
        return 0;
    }
    
    // Create a mutable copy of the list to sort
    let mut sorted = list.to_vec();
    sorted.sort();
    
    let len = sorted.len();
    if len % 2 == 1 {
        // Odd number of elements - return the middle one
        sorted[len / 2]
    } else {
        // Even number of elements - return average of the two middle ones
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2
    }
}

pub fn mode(list: &[i32]) -> i32 {
    if list.is_empty() {
        return 0;
    }
    
    // Use a HashMap to count occurrences of each number
    use std::collections::HashMap;
    let mut counts = HashMap::new();
    
    // Count occurrences
    for &num in list {
        *counts.entry(num).or_insert(0) += 1;
    }
    
    // Find the number with the highest count
    let mut max_count = 0;
    let mut mode_value = 0;
    
    for (&number, &count) in counts.iter() {
        if count > max_count {
            max_count = count;
            mode_value = number;
        }
    }
    
    mode_value
}
