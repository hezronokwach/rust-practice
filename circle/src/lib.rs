#[derive(Debug, Clone, Copy)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

impl Circle {
    pub fn new(x: f64, y: f64, r: f64) -> Self {
        Self {
            center: Point(x, y),
            radius: r,
        }
    }
    pub fn diameter(&self) -> f64 {
        self.radius * 2.0
    }
    pub fn area(&self) -> f64 {
        use std::f64::consts::PI;
        PI * self.radius * self.radius
    }
    pub fn intersect(&self, other: Circle) -> bool {
        let distance = ((self.center.0 - other.center.0).powi(2)
            + (self.center.1 - other.center.1).powi(2))
        .sqrt();

        distance <= self.radius + other.radius
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Point(pub f64, pub f64);

impl Point {
    pub fn distance(&self, other: Point) -> f64 {
        ((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2)).sqrt()
    }
}
