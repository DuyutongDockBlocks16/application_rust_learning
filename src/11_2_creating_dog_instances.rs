use rand::{thread_rng, Rng};

pub struct Dog {
    name: String,
    birth_year: u32,
    weight: f32,
}

pub fn rand_u32(min: u32, max: u32) -> u32 {
    return thread_rng().gen_range(min..=max);
}

pub fn rand_f32(min: f32, max: f32) -> f32 {
    return thread_rng().gen_range(min..=max);
}

pub fn rand_choice(choices: &[&str]) -> String {
    let i = thread_rng().gen_range(0..choices.len());
    choices[i].to_owned()
}

fn random_dog() -> Dog {
    // return a random dog, you may use the above helper functions
    let rand_dog = Dog{
        name: rand_choice(&["echo","cat","bird"]),
        birth_year: rand_u32(2000,2023),
        weight: rand_f32(10.0,100.0),
    };

    rand_dog
}

fn main() {
    let dog = random_dog();

    println!("Dog:");
    println!("  name: {}", dog.name);
    println!("  birth year: {}", dog.birth_year);
    println!("  weight: {:.2}", dog.weight);
}
