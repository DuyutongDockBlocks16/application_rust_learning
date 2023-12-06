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

#[derive(Debug, Clone, PartialEq)]
struct Player {
    score: u32,
    location: Point2d,
}

trait Location {
    fn location(&self) -> Point2d;
    fn collides_with(&self, other: &Self) -> bool {
        self.location() == other.location()
    }
    fn x(&self) -> f64 {
        self.location().x
    }
    fn y(&self) -> f64 {
        self.location().y
    }
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
        self.location
    }
}

trait Move:Location {
    fn move_to(&mut self, x: f64, y: f64);
    fn move_left(&mut self, amount: f64);
    fn move_right(&mut self, amount: f64);
    fn move_up(&mut self, amount: f64);
    fn move_down(&mut self, amount: f64);
}

impl Move for Player {
    fn move_to(&mut self, x: f64, y: f64) {
        self.location.x = x;
        self.location.y = y;
    }

    fn move_left(&mut self, amount: f64) {
        self.location.x = self.location.x-amount;
    }

    fn move_right(&mut self, amount: f64) {
        self.location.x = self.location.x+amount;
    }

    fn move_up(&mut self, amount: f64) {
        self.location.y = self.location.y+amount;
    }

    fn move_down(&mut self, amount: f64) {
        self.location.y = self.location.y-amount;
    }
}

fn main() {
    let mut player = Player::default();
    println!("the default player location is at ({}, {})", player.x(), player.y());
    let player2 = Player::default();
    println!("player and player2 collide? {}", player.collides_with(&player2));
    player.move_left(5.0);
    println!("player is now at ({}, {})", player.x(), player.y());
    println!("player and player2 collide? {}", player.collides_with(&player2));
}
