#[derive(Debug)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl Numbers {
    pub fn new(numbers: &[u32]) -> Self {
        Self{numbers}
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        Some(self.numbers[self.numbers.len()-1])
    }

    pub fn highest(&self) -> Option<u32> {
        let result = self.number.sort();
        Some(result.last())
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut result = self.number.sort();
        result.reverse();
        result[0..3].to_vec()
    }
}