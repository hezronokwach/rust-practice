
pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut mul: Vec<u32> = Vec::new();
    
    if s.chars().last().unwrap() == 'k' {
        // Remove the 'k' at the end before parsing
        let num_str = &s[0..s.len()-1];
        if let Ok(num) = num_str.parse::<u32>() {
            mul.push(num * 1000);
        }
    } else {
        // Handle case where string doesn't end with 'k'
        if let Ok(num) = s.parse::<u32>() {
            mul.push(num);
        }
    }
    
    Box::new(mul)
}

pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}
