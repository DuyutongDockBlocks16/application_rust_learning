use std::fmt::Debug;

fn sorted<T>(values: &[T] ) -> Vec<T>
    where
        T: Debug + PartialOrd + Clone + std::cmp::Ord
{
    let mut sorted_values = values.clone().to_vec();

    sorted_values.sort();
    sorted_values

}

fn main() {
    let vec = vec![10, -5, 30];
    let ar = ["🦫", "🦦", "🦥", "🦨", "🦡"];

    println!("{:?}", sorted(&vec));
    println!("{:?}", sorted(&ar));
}
