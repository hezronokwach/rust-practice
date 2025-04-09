pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut result = Vec::with_capacity(arr.len() + 1);
    
    // Calculate the sum for each partition
    let mut sum;
    for i in 0..=arr.len() {
        // For each partition, calculate the sum of elements from 0 to i-1
        sum = 0;
        for j in 0..i {
            sum += arr[j];
        }
        result.push(sum);
    }
    
    result.reverse();
    result
}