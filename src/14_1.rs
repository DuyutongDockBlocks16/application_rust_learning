macro_rules! S {
    ($element:expr) => {
        {
            let string = $element.to_string();
            string
        }
    };
}

fn main() {
    let my_owned_sweets = vec![S!("🍬"), S!("🍭"), S!("🍫"), S!("🍩"), S!("🍪")];
    println!("{my_owned_sweets:?}");
}
