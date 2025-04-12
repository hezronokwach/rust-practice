pub fn talking(text: &str) -> &'static str {
    // Handle empty input
    if text.is_empty() {
        return "Just say something!";
    }
    
    // Check if the text is all uppercase (yelling)
    let is_yelling = text.chars().any(|c| c.is_alphabetic()) && 
                     text.chars().filter(|c| c.is_alphabetic()).all(|c| c.is_uppercase());
    
    // Check if the text ends with a question mark
    let is_question = text.trim_end().ends_with('?');
    
    match (is_yelling, is_question) {
        (true, true) => "Quiet, I am thinking!",
        (true, false) => "There is no need to yell, calm down!",
        (false, true) => "Sure.",
        (false, false) => "Interesting"
    }
}