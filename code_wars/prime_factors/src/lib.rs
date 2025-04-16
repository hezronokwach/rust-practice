
pub fn prime_factors(n: u64) -> Vec<u64> {
    let mut digit = n;
    let mut factors = Vec::new();
    
    // Handle factor 2 separately for optimization
    while digit % 2 == 0 {
        factors.push(2);
        digit /= 2;
    }
    
    // Check odd numbers starting from 3
    let mut i = 3;
    while i * i <= digit {
        while digit % i == 0 {
            factors.push(i);
            digit /= i;
        }
        i += 2; // Only check odd numbers
    }
    
    // If digit is greater than 1, it's a prime factor
    if digit > 1 {
        factors.push(digit);
    }
    
    factors
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_prime_factors() {
        assert_eq!(prime_factors(2), vec![2]);
        assert_eq!(prime_factors(3), vec![3]);
        assert_eq!(prime_factors(4), vec![2, 2]);
        assert_eq!(prime_factors(12), vec![2, 2, 3]);
        assert_eq!(prime_factors(15), vec![3, 5]);
        assert_eq!(prime_factors(17), vec![17]);
        assert_eq!(prime_factors(13195), vec![5, 7, 13, 29]);
        assert_eq!(prime_factors(600851475143), vec![71, 839, 1471, 6857]);
    }
}
