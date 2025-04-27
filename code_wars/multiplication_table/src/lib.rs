fn multiplication_table(size: i32) -> Vec<Vec<i32>> {
    let mut table: Vec<Vec<i32>> = Vec::new();
    
    for i in 1..=size {
        let mut row: Vec<i32> = Vec::new();
        for j in 1..=size {
            row.push(i * j);
        }
        table.push(row);
    }
    
    table
}