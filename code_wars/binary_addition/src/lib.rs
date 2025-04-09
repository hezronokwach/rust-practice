fn add_binary(a: u64, b: u64) -> String {
    let add = a +b;
    let sum = format!("{:b}", add);
    sum
}
/*
fn add_binary(a: u64, b: u64) -> String {
    let mut number = a+b;
    
    let mut res_str = String::new();
    loop {
        let remainder = (number % 2) as u64;
        number = (number / 2) as u64;
        res_str = format!("{}{}", remainder, res_str);
        
        if number == 0 {  break; }
    }
    
    res_str
}
*/

#[cfg(test)]
mod tests {
    use super::add_binary;
    
    fn dotest(a: u64, b: u64, expected: &str) {
        let actual = add_binary(a, b);
        assert!(actual == expected, 
            "With a = {a}, b = {b}\nExpected \"{expected}\" but got \"{actual}\"")
    }

    #[test]
    fn sample_tests() {
        dotest(1,1,"10");
        dotest(0,1,"1");
        dotest(1,0,"1");
        dotest(2,2,"100");
        dotest(51,12,"111111");
    }
}