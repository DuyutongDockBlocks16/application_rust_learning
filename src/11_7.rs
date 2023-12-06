#[derive(Debug)]
struct Point2d {
    x: f64,
    y: f64,
}

impl Point2d {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    fn from_tuple(tuple: (f64, f64)) -> Self {
        Self::new(tuple.0, tuple.1)
    }

    fn add(&self, other: &Self) -> Self {
        Self::new(self.x + other.x, self.y + other.y)
    }

    fn multiply(&self, c: f64) -> Self {
        Self::new(self.x * c, self.y * c)
    }

    // Implement method add_inplace
    fn add_inplace(&mut self, other: &Self){
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }

    // Implement method multiply_inplace
    fn multiply_inplace(&mut self, c: f64) {
        self.x = self.x * c;
        self.y = self.y * c;
    }
}

fn main() {
    let mut direction = Point2d::new(1.0, 0.0);
    let velocity = 5.5;
    // let movement = direction.multiply(velocity);
    direction.multiply_inplace(velocity);

    let mut triangle = [(1.2, 1.1), (5.8, 10.9), (6.5, -3.5)]
        .into_iter()
        .map(Point2d::from_tuple)
        .collect::<Vec<_>>();

    println!("Triangle coordinates:");
    triangle
        .iter()
        .for_each(|point| println!("  x: {}, y: {}", point.x, point.y));

    println!(
        "Moving triangle by: x: {}, y: {}...",
        direction.x, direction.y
    );

    triangle
        .iter_mut()
        .for_each(|point| point.add_inplace(&direction));

    println!("Triangle coordinates:");
    triangle
        .iter()
        .for_each(|point| println!("  x: {}, y: {}", point.x, point.y));
}
