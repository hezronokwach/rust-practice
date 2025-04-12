pub fn is_pangram(s: &str) -> bool {
    let mut store : String  = "abcdefghijklmnopqrstuvwxyz".to_string();
    for c in s.chars() {
        if store.contains(c.to_ascii_lowercase()){
           let index = store.find(c).unwrap();
           store.remove(index);
        }
    }
    store.len() == 0    
}