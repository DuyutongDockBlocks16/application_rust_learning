#[derive(Debug,Clone)]
struct Cat {
    name: String,
    age: u32,
    weight: f32,
}

fn main() {
    let cat = Cat {
        name: "Tom".to_string(),
        age: 3,
        weight: 5.0,
    };
    let copycat = cat.clone();

    println!("Cat:");
    println!("{cat:#?}");
    println!("Copycat:");
    println!("{copycat:#?}");
}
