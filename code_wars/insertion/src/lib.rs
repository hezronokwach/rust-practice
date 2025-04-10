pub fn insertion_sort(slice: &mut [i32], steps: usize) {
    let len = slice.len();
    
    // Handle empty or single-element slices
    if len <= 1 {
        return;
    }
    
    // Limit steps to the maximum possible iterations (len - 1)
    let actual_steps = steps.min(len - 1);
    
    // Perform the insertion sort algorithm for the specified number of steps
    for i in 0..actual_steps {
        // The current position we're working with
        let current_pos = i + 1;
        
        // Store the current value that needs to be inserted
        let current_value = slice[current_pos];
        
        // Find the correct position to insert the current value
        let mut j = current_pos;
        
        // Move elements greater than current_value one position ahead
        while j > 0 && slice[j - 1] > current_value {
            slice[j] = slice[j - 1];
            j -= 1;
        }
        
        // Insert the current value at its correct position
        slice[j] = current_value;
    }
}

