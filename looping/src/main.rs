use std::io;

fn main() {
    let mut count = 0;
    let answer = "The letter e";

    loop {
        count = count + 1;
        
        println!("I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?");
        
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        
        let input = input.trim().to_lowercase();
        
        match input.as_str() {
            s if s == answer => {
                println!("The letter e");
                println!("Number of trials {}", count);
                break;
            }
            _ => {
                continue;
            }
        }
    }
}