use application::PositiveF64;

struct Cat {
    name: String,
    age: u32,
    weight: PositiveF64,
}

impl Cat {
    fn new(name: &str, age: u32, weight: PositiveF64) -> Self {
        Self {
            name: name.to_string(),
            age,
            weight,
        }
    }
}

fn main() {
    let mut cat = Cat::new("Cheshire", 4, PositiveF64::new(3.5).unwrap());
    println!("{} weighs {}", cat.name, cat.weight.value());
    println!("A year goes by... 🐈");
    cat.age += 1;
    cat.weight.replace_value(cat.weight.value() + 0.5);

    println!("{} weighs {}", cat.name, cat.weight.value());
}
