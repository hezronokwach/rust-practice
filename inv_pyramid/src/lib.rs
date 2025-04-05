pub fn inv_pyramid(v: String, i: u32) -> Vec<String> {
    let mut result = Vec::new();
    
    // Handle the case where i is 0
    if i == 0 {
        return result;
    }
    
    // Generate the increasing part of the pyramid (including the peak)
    for j in 1..=i {
        // Create a string with j spaces followed by j repetitions of v
        let spaces = " ".repeat(j as usize);
        let chars = v.repeat(j as usize);
        result.push(spaces + &chars);
    }
    
    // Generate the decreasing part of the pyramid
    for j in (1..i).rev() {
        // Create a string with j spaces followed by j repetitions of v
        let spaces = " ".repeat(j as usize);
        let chars = v.repeat(j as usize);
        result.push(spaces + &chars);
    }
    
    result
}