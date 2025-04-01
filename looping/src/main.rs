use std::io;

fn main() {
    let mut trials = 0;
    let mut ans = String::new();
    println!(
        "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
    );
    loop {
        ans.clear();
        io::stdin().read_line(&mut ans).expect("Failed input");
        trials += 1;
        let ans2 = ans.trim().to_lowercase();
        if ans2 == "e" || ans2 == "The letter e"{      
            println!("The letter e");
            println!("Number of trials {}", trials);
            break;
        }else{
            println!(
                "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?"
            );
        }
    }
}
