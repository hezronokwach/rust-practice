pub fn first_subword(mut s: String) -> String {
    for (i, c) in s.chars().enumerate(){
        if i != 0 && c.is_uppercase() ||c == '_' {
            s.truncate(i);
            break;
        }
    }
    s    
}