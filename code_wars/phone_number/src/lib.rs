fn create_phone_number(numbers: &[i8]) -> String {
    let mut result = String::new();
    
    result.push('(');
    for (i, &num) in numbers.iter().enumerate() {
        result.push((num as u8 + b'0') as char);
        if i == 2 {
            result.push_str(") ");
        } else if i == 5 {
            result.push('-');
        }
    }
    
    result
}