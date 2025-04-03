use::std::collections::HashMap;
pub fn bigger(h: HashMap<&str, i32>) -> i32 {
    let mut store :Vec<i32>= Vec::new();
    for &values in h.values(){
        store.push(values);
    }
    store.sort();
    store[store.len()-1]
}