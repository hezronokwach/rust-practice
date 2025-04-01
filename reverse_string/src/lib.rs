pub fn rev_str(input: &str) -> String {
    let mut result = String::new();
    for word in input.chars().rev(){
        result.push(word);
    } 
    result  
}