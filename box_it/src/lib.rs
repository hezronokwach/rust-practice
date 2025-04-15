pub fn transform_and_save_on_heap(s: String) -> Box<Vec<u32>> {
    let mut mul: Vec<u32> = Vec::new();
    if s.chars().last().unwrap() == 'k' {
        let num: u32 = s.parse().unwrap();
        mul.push(num * 1000 as u32);
    }
    let store = Box::new(mul);
    store
}
pub fn take_value_ownership(a: Box<Vec<u32>>) -> Vec<u32> {
    *a
}
