#![allow(unused)]

#[derive(Debug, PartialEq, Eq, Default)]
enum Status {
    #[default]
    Offline,
    Online,
}

#[derive(Debug, Default)]
struct User {
    name: String,
    status: Status,
}

fn main() {
    let user = User::default();
    println!("{:?}", user);
}
