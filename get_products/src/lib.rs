pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let n = arr.len();
    let mut result = vec![1; n];

    // Handle edge cases
    if n == 0 || n == 1 {
        return Vec::new();
    }

    // First pass: Compute products of all elements to the left of each index
    let mut left_product = 1;
    for i in 0..n {
        result[i] = left_product;
        left_product *= arr[i];
    }

    // Second pass: Multiply by products of all elements to the right
    let mut right_product = 1;
    for i in (0..n).rev() {
        result[i] *= right_product;
        right_product *= arr[i];
    }

    result
}