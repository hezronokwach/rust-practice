pub fn talking(text: &str) -> &str {
    if text.is_empty(){
        return "Just say something!";
    }
    for (i,char) in text.chars().enumerate() {
        if i+1 < text.len() && char.is_uppercase() && !text.chars().nth(i+1).unwrap().is_lowercase() {
            if text.chars().last().unwrap() == '!' {
                return "There is no need to yell, calm down!";
            } else if text.chars().last().unwrap() == '?' {
                return "Quiet, I am thinking!";
            }
        } else if text.chars().last().unwrap() == '?' {
            return "Sure";

        }
    }
    "Interesting"
}
