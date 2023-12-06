#[derive(Copy, Clone, Debug)]
struct Miles(f32);

#[derive(Copy, Clone, Debug)]
struct Kilometers(f32);

const KILOMETERS_PER_MILE: f32 = 1.609344;

// Implementing PartialEq for Miles against Kilometers
impl PartialEq<Kilometers> for Miles {
    fn eq(&self, other: &Kilometers) -> bool {
        self.0 * KILOMETERS_PER_MILE == other.0
    }
}

// Implementing PartialEq for Kilometers against Miles
impl PartialEq<Miles> for Kilometers {
    fn eq(&self, other: &Miles) -> bool {
        self.0 == other.0 * KILOMETERS_PER_MILE
    }
}

fn main() {
    let miles = Miles(1.0);
    let kilometers = Kilometers(1.609344);
    println!("{}", miles == kilometers);
    println!("{}", kilometers != miles);
}
