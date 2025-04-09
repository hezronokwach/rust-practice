fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    sequence.into_iter().
    fold(Vec::new(), |mut acc, item| {
        if acc.is_empty() || acc.last().unwrap() != &item {
            acc.push(item);
        }
        acc
    })
}


/*
fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut v: Vec<_> = sequence.into_iter().collect();
    v.dedup();
    v
}


fn unique_in_order<T>(sequence: T) -> Vec<T::Item>
where
    T: std::iter::IntoIterator,
    T::Item: std::cmp::PartialEq + std::fmt::Debug,
{
    let mut result: Vec<T::Item> = Vec::new();
    let mut last_element = None;
    
    for element in sequence {
        match &last_element {
            // If we have no last element or the current element is different from the last one
            None => {
                result.push(element);
                last_element = Some(element);
            },
            Some(last) if *last != element => {
                result.push(element);
                last_element = Some(element);
            },
            // If the element is the same as the last one, skip it
            _ => {}
        }
    }
    
    result
} */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(unique_in_order("AAAABBBCCDAABBB".chars()), vec!['A','B','C','D','A','B']);
    }
}