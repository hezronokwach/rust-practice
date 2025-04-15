pub fn search(array: &[i32], key: i32) -> Option<usize> {
    for (i, c) in array.iter().enumerate() {
        if *c == key {
            return Some(i);
        }
    }
    None
}
