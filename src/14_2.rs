use std::fmt;

struct Cat {
    name: String,
    weight: f64,
    age: u8,
}

impl fmt::Display for Cat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

struct Dog {
    name: String,
    weight: f64,
    age: u8,
}

struct Mouse {
    name: String,
    weight: f64,
    age: u8,
}

macro_rules! impl_display {
    ($struct_name:ident, $field_name:ident) => {
        impl fmt::Display for $struct_name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.$field_name)
            }
        }
    };
}

// impl_display!(Cat, name);
impl_display!(Dog, name);
impl_display!(Mouse, name);



fn main() {
    let cat = Cat {
        name: "Felix".to_string(),
        weight: 4.5,
        age: 5,
    };

    let dog = Dog {
        name: "Innocentius".to_string(),
        weight: 30.5,
        age: 3,
    };

    let mouse = Mouse {
        name: "Hilarius".to_string(),
        weight: 0.03,
        age: 1,
    };
    println!("{cat}");
    println!("{dog}");
    println!("{mouse}");
}
