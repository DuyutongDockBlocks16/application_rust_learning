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

trait Location {
    fn location(&self) -> Point2d;
    fn collides_with(&self, other: &Self) -> bool;
    fn x(&self) -> f64;
    fn y(&self) -> f64;
}

#[derive(Debug, Clone, PartialEq)]
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

impl Default for Player {
    fn default() -> Self {
        Self::new(0, 10.0, 5.0)
    }
}

impl Location for Player {
    fn location(&self) -> Point2d {
        Point2d::new(self.location.x, self.location.y)
    }

    fn collides_with(&self, other: &Self) -> bool {
        if self.location.x==other.location.x && self.location.y==other.location.y{
            return true
        }
        return false
    }

    fn x(&self) -> f64 {
        self.location.x
    }

    fn y(&self) -> f64 {
        self.location.y
    }
}

fn main() {
    let player = Player::default();
    println!("the default player location is {:?}", player.location());
    println!(
        "the default player location is at ({}, {})",
        player.x(),
        player.y()
    );
    let player2 = Player::default();
    println!(
        "player and player2 collide? {}",
        player.collides_with(&player2)
    );
}
