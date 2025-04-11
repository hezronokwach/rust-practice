pub fn score(input :&str) -> u64 {
    let mut sum = 0;
    let first = "AEIOULNRST";
    let second = "DG";
    let third = "BCMP" ;
    let fourth = "FHVWY";
    let fifth = "K";
    let sixth = "JX";
    let seventh = "QZ";
    for x in input.chars()    
    {
        if first.contains(x.to_ascii_uppercase()){
            sum += 1;
        } else if second.contains(x.to_ascii_uppercase()){
            sum += 2;
        } else if third.contains(x.to_ascii_uppercase()){
            sum += 3;
        } else if fourth.contains(x.to_ascii_uppercase()){
            sum += 4;
        } else if fifth.contains(x.to_ascii_uppercase()){
            sum += 5;
        } else if sixth.contains(x.to_ascii_uppercase()) {
            sum +=8;
        } else if seventh.contains(x.to_ascii_uppercase()) {
            sum += 10;
        }
    }
sum as u64
}