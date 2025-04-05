pub fn bubble_sort(arr: &mut [i32]) {
    let n = arr.len();

    // Early return for empty or single-element arrays
    if n <= 1 {
        return;
    }

    for i in 0..n {
        let mut swapped = false;

        for j in 0..n - i - 1 {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
