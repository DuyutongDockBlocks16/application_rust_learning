struct Point2d {
    x: f64,
    y: f64,
}

impl Point2d {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl From<(f64, f64)> for Point2d {
    fn from(value: (f64, f64)) -> Self {
        Point2d {
            x: value.0,
            y: value.1,
        }
    }
}

fn main() {
    let point = Point2d::new(0.0, 0.0);

    let points = [(1.2, 1.1), (5.8, 10.9)];
    let points = points.map(Point2d::from);

    println!("x: {}, y: {}", point.x, point.y);
    points
        .iter()
        .for_each(|point| println!("x: {}, y: {}", point.x, point.y));
}
