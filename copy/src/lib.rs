pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let d = c as f64;
    (c, d.exp(), d.abs().ln())
}

pub fn str_function(a: String) -> (String, String) {
    let numbers: Vec<f64> = a
        .split_whitespace()
        .map(|s| s.parse::<f64>().unwrap())
        .collect();
    
    let exp_nums: Vec<String> = numbers
        .iter()
        .map(|&n| n.exp().to_string())
        .collect();
    
    (a, exp_nums.join(" "))
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let result: Vec<f64> = b
        .iter()
        .map(|&x| (x as f64).abs().ln())
        .collect();
    
    (b, result)
}