#[derive(Debug)]
struct Point2d {
    x: f64,
    y: f64,
}

// implement associated functions new and from_tuple for point2d
impl Point2d {
    fn new(x:f64, y:f64) -> Point2d{
        Point2d {x:x, y:y}
    }

    fn from_tuple(location_tuple:(f64, f64)) -> Point2d{
        Point2d {x:location_tuple.0, y:location_tuple.1}
    }
}


fn main() {
    let point = Point2d::new(0.0, 0.0);

    let points = [(1.2, 1.1), (5.8, 10.9)];
    let points = points.map(Point2d::from_tuple);

    println!("{point:?}");
    points.iter().for_each(|point| println!("{point:?}"));
}