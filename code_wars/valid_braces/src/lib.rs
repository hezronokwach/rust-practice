fn valid_braces(s: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for ch in s.chars() {
        if ch == '(' || ch == '{' || ch == '[' {
            stack.push(ch);
        }
        if ch == ')' {
            if stack.is_empty() || *stack.last().unwrap() != '(' {
                return false;
            }
            stack.pop().unwrap();
        } else if ch == '}' {
            if stack.is_empty() || *stack.last().unwrap() != '{' {
                return false;
            }
            stack.pop().unwrap();
        } else if ch == ']' {
            if stack.is_empty() || *stack.last().unwrap() != '[' {
                return false;
            }
            stack.pop().unwrap();
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        expect_true("()");
        expect_false("[(])");
    }

    fn expect_true(s: &str) {
        assert!(
            valid_braces(s),
            "Expected {s:?} to be valid. Got false",
            s = s
        );
    }

    fn expect_false(s: &str) {
        assert!(
            !valid_braces(s),
            "Expected {s:?} to be invalid. Got true",
            s = s
        );
    }
}
