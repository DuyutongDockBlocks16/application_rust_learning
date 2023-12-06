use std::fmt::Debug;

fn debug(value: &impl Debug) {
    println!("{:?}",value)
}

fn main() {
    let tuple = (1, 2, 3);
    let vec = vec!["🐛", "🐝", "🐞"];
    let int = 1;

    debug(&tuple);
    debug(&vec);
    debug(&int);
}
