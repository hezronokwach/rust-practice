use std::io;

fn main() {
    let mut trials = 0;
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    
    println!("{}", riddle);
    
    loop {
        let mut input = String::new();
        
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                trials += 1;
                let answer = input.trim().to_lowercase();
                
                if answer == "e" || answer == "the letter e" {
                    println!("The letter e");
                    println!("Number of trials {}", trials);
                    break;
                } else {
                    println!("{}", riddle);
                }
            },
            Err(_) => {
                println!("Error reading input. Please try again.");
            }
        }
    }
}