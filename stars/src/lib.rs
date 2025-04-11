pub fn stars(n: u32) -> String {
    let mut result = String::new();
    for _ in 0 .. 2u32.pow(n as u32) {
        result.push('*') ;   
    }
result
}