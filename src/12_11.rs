use std::ops::{Add, Mul};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point2d {
    x: f64,
    y: f64,
}

impl From<(f64, f64)> for Point2d {
    fn from(t: (f64, f64)) -> Self {
        Point2d::new(t.0, t.1)
    }
}

impl Point2d {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Self) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}



// Implementing Add for Point2d
impl Add for Point2d {
    type Output = Point2d;

    fn add(self, other: Point2d) -> Point2d {
        Point2d {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Implementing Mul for Point2d with f64 as RHS
impl Mul<f64> for Point2d {
    type Output = Point2d;

    fn mul(self, scalar: f64) -> Point2d {
        Point2d {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// ... existing main function ...
fn main() {
    let direction = Point2d::new(1.0, 0.0);
    let velocity = 5.5;
    let movement = direction * velocity;

    let triangle = [(1.2, 1.1), (5.8, 10.9), (6.5, -3.5)];
    // Move the triangle
    let triangle = triangle.map(Point2d::from).map(|point| point + movement);

    println!("Triangle coordinates:");
    triangle.iter().for_each(|point| println!("{:?}", point));
    println!("Side widths:");
    triangle
        .windows(2)
        .map(|points| points[0].distance(&points[1]))
        .for_each(|w| println!("  {:?}", w));
    println!("  {:?}", triangle[2].distance(&triangle[0]));
}
