fn spin_words(words: &str) -> String {
    let store = words.split_whitespace().collect::<Vec<&str>>();
    let mut result = String::new();
    for word in &store {
        if word.len() >= 5 {
            for charss in word.chars().rev() {
                result.push(charss);
            }
            if store.len() > 1 as usize && *word != store[store.len()-1]{
                result.push(' ');
            }
        } else {
            result.push_str(word);
            if store.len() > 1 && *word != store[store.len()-1] {
                result.push(' ');
            }
        }
    }
    result
}
/*
fn spin_words(words: &str) -> String {
    words.split_ascii_whitespace().map(|word| match word.len() >= 5 {
        true => word.chars().rev().collect(),
        false => word.to_string()
    }).collect::<Vec<String>>().join(" ")
}

*/


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples() {
        assert_eq!(spin_words("Welcome"), "emocleW");
        assert_eq!(spin_words("Hey fellow warriors"), "Hey wollef sroirraw");
        assert_eq!(spin_words("This is a test"), "This is a test");
        assert_eq!(spin_words("This is another test"), "This is rehtona test");
        assert_eq!(
            spin_words("You are almost to the last test"),
            "You are tsomla to the last test"
        );
        assert_eq!(
            spin_words("Just kidding there is still one more"),
            "Just gniddik ereht is llits one more"
        );
        assert_eq!(
            spin_words("Seriously this is the last one"),
            "ylsuoireS this is the last one"
        );
    }
}
