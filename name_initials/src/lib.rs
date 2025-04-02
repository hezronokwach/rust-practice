pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result = Vec::new();
    
    for name in names {
        let mut initial = String::new();
        
        // Get first character
        if let Some(first) = name.chars().next() {
            initial.push(first);
            initial.push_str(". ");
        }
        
        // Find character after space
        if let Some(space_idx) = name.find(' ') {
            if let Some(second) = name[space_idx..].chars().nth(1) {
                initial.push(second);
                initial.push_str(".");
            }
        }
        
        result.push(initial);
    }
    
    result
}