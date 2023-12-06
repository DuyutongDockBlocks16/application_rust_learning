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

    // Implement method add
    fn add(&self,movement: &Point2d) -> Self{
        Self::new(self.x + movement.x, self.y + movement.y )
    }

    // Implement method multiply
    fn multiply(&self,velocity:f64) -> Self{
        Self::new(self.x * velocity, self.y * velocity )
    }

}

fn main() {
    let direction = Point2d::new(1.0, 0.0);
    let velocity = 5.5;
    let movement = direction.multiply(velocity);

    let triangle = [(1.2, 1.1), (5.8, 10.9), (6.5, -3.5)]
        .into_iter()
        .map(Point2d::from_tuple)
        .collect::<Vec<_>>();

    println!("Triangle coordinates:");
    triangle
        .iter()
        .for_each(|point| println!("  x: {}, y: {}", point.x, point.y));

    println!(
        "Moving triangle in direction: x: {}, y: {} with velocity {velocity}",
        direction.x, direction.y
    );

    let triangle = triangle
        .iter()
        .map(|point| point.add(&movement))
        .collect::<Vec<_>>();

    println!("Triangle coordinates:");
    triangle
        .iter()
        .for_each(|point| println!("  x: {}, y: {}", point.x, point.y));
}
