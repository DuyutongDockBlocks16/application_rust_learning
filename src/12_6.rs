
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Pair<L,R>{
    left: L,
    right: R
}

impl<L,R> Pair<L,R>{
    fn new(left:L ,right:R) -> Self{
        Pair {left,right}
    }
}

fn main() {
    let pair = Pair::new("🍐", 1);
    println!("{:?}", pair);
}
