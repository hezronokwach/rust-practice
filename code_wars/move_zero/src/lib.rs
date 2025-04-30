fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut i = 0;
    let mut v = vec![0;arr.len()];
    for &e in arr {
        if e!=0 {
            v[i] = e;
            i += 1;
        }
    }
    v
}