#[derive(Debug, Clone)]
pub struct Cat {
    name: String,
    age: u32,
    weight: f32,
}

// implement the function for renaming a cat
fn rename(cat: &mut Cat, new_name: &str){
    cat.name = new_name.to_string();
}


fn main() {
    let mut cat = Cat {
        name: "Tom".to_string(),
        age: 3,
        weight: 5.0,
    };

    let copycat = cat.clone();
    rename(&mut cat, "Whiskers");
    cat.age += 1;
    cat.weight -= 0.5;

    println!("Cat:");
    println!("{:?}", cat);
    println!("Copycat:");
    println!("{:?}", copycat);
}
