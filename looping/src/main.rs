use std::io;

fn main() {
    let riddle = "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
    let mut trials = 0;
    
    println!("{}", riddle);
    
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        trials += 1;
        
        if input.trim().to_lowercase() == "e" {
            println!("The letter e");
            println!("Number of trials {}", trials);
            break;
        } else {
            println!("{}", riddle);
        }
    }
}