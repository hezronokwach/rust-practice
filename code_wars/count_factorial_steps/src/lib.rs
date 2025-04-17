pub fn count_factorial_steps(factorial: u64) -> u64 {
    // Handle special cases: 0 and 1
    if factorial <= 1 {
        return 0;
    }
    
    let mut remaining = factorial;
    let mut divisor = 2; // Start dividing by 2
    let mut steps = 0;
    
    // Keep dividing by increasing divisors until we reach 1
    // or determine it's not a factorial
    while remaining > 1 {
        if remaining % divisor != 0 {
            // If not divisible, it's not a factorial
            return 0;
        }
        
        remaining /= divisor;
        steps += 1;
        divisor += 1;
    }
    
    steps
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_count_factorial_steps() {
        // Test cases from the example
        assert_eq!(count_factorial_steps(720), 6); // 6! = 720
        assert_eq!(count_factorial_steps(13), 0);  // Not a factorial
        assert_eq!(count_factorial_steps(6), 3);   // 3! = 6
        
        // Additional test cases
        assert_eq!(count_factorial_steps(0), 0);   // Special case
        assert_eq!(count_factorial_steps(1), 0);   // Special case (0! = 1! = 1)
        assert_eq!(count_factorial_steps(2), 2);   // 2! = 2
        assert_eq!(count_factorial_steps(24), 4);  // 4! = 24
        assert_eq!(count_factorial_steps(120), 5); // 5! = 120
        assert_eq!(count_factorial_steps(3628800), 10); // 10! = 3628800
    }
}