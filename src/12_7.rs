#[derive(Debug)]
pub struct Pair<T, U> {
    left: T,
    right: U,
}

impl<T, U> Pair<T, U>
    where
        T:Default+Clone,
        U:Default+Clone
{
    pub fn new(first: T, second: U) -> Self {
        Self {
            left: first,
            right: second,
        }
    }
}

impl<T, U> Default for Pair<T, U>
    where
        T:Default+Clone,
        U:Default+Clone
{
    fn default() -> Self {
        Pair::new(T::default(),U::default())
    }
}

impl<T: Clone> Pair<T, T> {
    pub fn to_vec(&self) -> Vec<T> {
        vec![self.left.clone(), self.right.clone()]
    }
}

fn main() {
    let pair: Pair<i32, String> = None.unwrap_or_default();
    println!("{:?}", pair);
    let pair = Pair::new("🍐", "🍎");
    println!("{:?}", pair.to_vec());
}