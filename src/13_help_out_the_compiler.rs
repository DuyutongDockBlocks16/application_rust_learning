use std::collections::HashMap;
use std::hash::Hash;

fn longest_common_prefix<'a>(a: &'a str, b: &'a str) -> &'a str {
    let common_prefix_length = a.chars().zip(b.chars()).take_while(|(a, b)| a == b).count();
    &a[..common_prefix_length]
}

fn reverse_index<'a, T: Eq + Hash + ?Sized>(input: &'a [&'a T]) -> HashMap<&'a T, usize> {
    input.iter().enumerate().map(|(i, s)| (*s, i)).collect()
}

fn main() {
    let input = ["flower", "flow", "flight"];
    let common_prexix = longest_common_prefix(input[0], input[1]);
    println!("{}", common_prexix);
    let common_prexix = longest_common_prefix(input[0], input[2]);
    println!("{}", common_prexix);

    let index = reverse_index(&input);
    println!("{:?}", index);
}
