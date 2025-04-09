pub fn min_and_max(nb_1: i32, nb_2: i32, nb_3: i32) -> (i32, i32) {
    let store : Vec<i32> = vec![nb_1,nb_2,nb_3];
    let max = store.iter().max().unwrap();
    let min = store.iter().min().unwrap();
    (*min ,*max)
}