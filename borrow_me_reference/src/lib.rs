pub fn delete_and_backspace(s: &mut String) {
    let mut i = 0;
    while i < s.len() {
        if i > 0 && s.chars().nth(i) == Some('-') {
            s.remove(i);
            s.remove(i - 1);
            i = i.saturating_sub(1);
        } else if s.chars().nth(i) == Some('+') {
            s.remove(i);
            if i < s.len() {
                s.remove(i);
            }
        } else {
            i += 1;
        }
    }
}

pub fn do_operations(v: &mut [String]) {
    for string in v.iter_mut() {
        let parts = string.split_whitespace(); // Iterator: ["1", "+", "2"]
        let vec = parts.collect::<Vec<_>>(); // Vector: ["1", "+", "2"]
        let tuple = vec.match_tuple(); // Option<("1", "+", "2")>
        if let Some((left, op, right)) = tuple {
            let left_num: i32 = left.parse().unwrap_or(0);
            let right_num: i32 = right.parse().unwrap_or(0);

            let result = match op {
                "+" => left_num + right_num,
                "-" => left_num - right_num,
                _ => 0,
            };

            *string = result.to_string();
        }
    }
}
