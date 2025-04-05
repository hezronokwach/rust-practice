pub fn is_permutation(s1: &str, s2: &str) -> bool {
    // Quick check: if lengths are different, they can't be permutations
    if s1.len() != s2.len() {
        return false;
    }

    // Use a HashMap to count character occurrences
    use std::collections::HashMap;
    
    let mut char_counts: HashMap<char, i32> = HashMap::new();
    
    // Count occurrences in the first string
    for c in s1.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }
    
    // Decrement counts for each character in the second string
    for c in s2.chars() {
        match char_counts.get_mut(&c) {
            Some(count) => {
                *count -= 1;
                if *count < 0 {
                    return false; // More occurrences in s2 than in s1
                }
            },
            None => return false, // Character in s2 not present in s1
        }
    }
    
    // All counts should be zero if strings are permutations
    char_counts.values().all(|&count| count == 0)
}