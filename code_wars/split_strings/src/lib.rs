fn solution(s: &str) -> Vec<String> {
    let mut store = String::new();
    let mut result: Vec<String> = Vec::new();
    
    for c in s.chars() {
        store.push(c);
        if store.len() == 2 {
            result.push(store);
            store = String::new();
        }
    }
    
    // Handle the last chunk if it's not empty
    if !store.is_empty() {
        store.push('_'); // Add underscore if length is odd
        result.push(store);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}