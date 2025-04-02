pub fn delete_and_backspace(s: &mut String) {
    let mut chars: Vec<char> = s.chars().collect();
    let mut i = chars.len();
    while i > 0 {
        i -= 1;
        if chars[i] == '+' {
            chars.remove(i);
            if i < chars.len() {
                chars.remove(i);
            }
        }
    }
    i = 0;
    while i < chars.len() {
        if chars[i] == '-' {
            if i > 0 {
                chars.remove(i - 1);
                i -= 1;
            }
            chars.remove(i);
        } else {
            i += 1;
        }
    }

    *s = chars.into_iter().collect();
}
pub fn do_operations(v: &mut [String]) {
    for string in v.iter_mut() {
        if let Some(pos) = string.find('+') {
            let left: i32 = string[..pos].parse().unwrap_or(0);
            let right: i32 = string[pos + 1..].parse().unwrap_or(0);
            *string = (left + right).to_string();
        } else if let Some(pos) = string.find('-') {
            let left: i32 = string[..pos].parse().unwrap_or(0);
            let right: i32 = string[pos + 1..].parse().unwrap_or(0);
            *string = (left - right).to_string();
        }
    }
}
