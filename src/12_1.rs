#[derive(Debug, Clone, Copy, PartialEq)]
struct Point2d {
    x: f64,
    y: f64,
}

impl Point2d {
    fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
struct Player {
    score: u32,
    location: Point2d,
}

impl Player {
    fn new(score: u32, x: f64, y: f64) -> Self {
        Self {
            score,
            location: Point2d::new(x, y),
        }
    }
}

impl Default for Player{
    fn default() -> Self {
        Self {
            score:0,
            location: Point2d::new(10.0, 5.0),
        }
    }
}

fn main() {
    let player = Player::default();

    println!(
        "The default player score is {} and location is at ({}, {})",
        player.score, player.location.x, player.location.y
    );
}
