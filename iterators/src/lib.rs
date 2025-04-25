#[derive(Copy, Clone)]
pub struct Collatz {
    pub v: u64,
}

impl Iterator for Collatz {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.v == 0 {
            return None;
        }

        let current = self.v;
        if current == 1 {
            self.v = 0; // Stop after yielding 1
        } else if current % 2 == 0 {
            self.v = current / 2;
        } else {
            self.v = current * 3 + 1;
        }

        Some(current)
    }
}

impl Collatz {
    pub fn new(n: u64) -> Self {
        Collatz { v: n }
    }
}

pub fn collatz(n: u64) -> usize {
    if n == 0 {
        return 0; // Invalid input, no steps
    }
    Collatz::new(n).count() - 1
}