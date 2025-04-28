pub fn prev_prime(nbr: u64) -> u64 {
    // Handle edge cases: 0 and 1 are not prime
    if nbr <= 1 {
        return 0;
    }

    // Start from nbr
    let mut i = nbr;
    while i >= 2 {
        if is_prime(i) {
            return i;
        }
        i -= 1;
    }

    // If no prime is found (shouldn't happen since we start at 2)
    0
}

// Assuming is_prime function exists; here's a sample implementation
fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    for i in (3..=(n as f64).sqrt() as u64).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    true
}