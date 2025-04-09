fn high_and_low(numbers: &str) -> String {
    let digits: Vec<&str> = numbers.split_whitespace().collect();
    let mut digit: Vec<i32> = Vec::new();
    let result = String::new();
    for number in digits {
        let conv: i32 = number.parse().unwrap();
        digit.push(conv);
    }
    digit.sort();
    let first = digit.first().unwrap();
    let last = digit.last().unwrap();
    format!("{} {}", last, first)
}

/*
   fn high_and_low(numbers: &str) -> String {
    let mut values: Vec<i32> = numbers
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    let max = values.iter().max().unwrap();
    let min = values.iter().min().unwrap();

    format!("{} {}", max, min)
} */
