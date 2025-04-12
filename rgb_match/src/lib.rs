
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        // Create a new color with the same values
        let mut result = self;
        
        // Swap the values based on which components match the input values
        if first == self.r {
            if second == self.g {
                result.r = self.g;
                result.g = self.r;
            } else if second == self.b {
                result.r = self.b;
                result.b = self.r;
            } else if second == self.a {
                result.r = self.a;
                result.a = self.r;
            }
        } else if first == self.g {
            if second == self.r {
                result.g = self.r;
                result.r = self.g;
            } else if second == self.b {
                result.g = self.b;
                result.b = self.g;
            } else if second == self.a {
                result.g = self.a;
                result.a = self.g;
            }
        } else if first == self.b {
            if second == self.r {
                result.b = self.r;
                result.r = self.b;
            } else if second == self.g {
                result.b = self.g;
                result.g = self.b;
            } else if second == self.a {
                result.b = self.a;
                result.a = self.b;
            }
        } else if first == self.a {
            if second == self.r {
                result.a = self.r;
                result.r = self.a;
            } else if second == self.g {
                result.a = self.g;
                result.g = self.a;
            } else if second == self.b {
                result.a = self.b;
                result.b = self.a;
            }
        }
        
        result
    }
}
