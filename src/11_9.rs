enum Exercise{
    OpenText,
    MultipleChoice
}

fn main() {
    let exercise = Exercise::OpenText;
    match exercise {
        Exercise::OpenText => println!("Open text exercise"),
        Exercise::MultipleChoice => println!("Multiple choice exercise"),
    }
}
