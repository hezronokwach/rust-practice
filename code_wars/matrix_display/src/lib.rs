pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let data = slice.iter().map(|row| row.to_vec()).collect::<Vec<Vec<i32>>>();
        Matrix(data)
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.0 {
            write!(f, "(")?;
            for (i, &num) in row.iter().enumerate() {
                if i > 0 {
                    write!(f, " ")?;
                }
                write!(f, "{}", num)?;
            }
            writeln!(f, ")")?;
        }
        Ok(())
    }
}