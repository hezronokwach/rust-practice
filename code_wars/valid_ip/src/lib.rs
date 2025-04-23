pub fn is_valid_ip(ip: &str) -> bool {
    if ip.len() < 7{
        return false;
    }
    let store  = ip.split('.');
    for word in store{
        if word.len() > 2 && word.chars().next() == Some('0'){
            return false
        }    
        let num = word.parse()::collect(i32);   
        if num < 0 || num > 255{
            return false;
        }
    }
    true     
}