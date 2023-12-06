pub fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

pub enum Exercise {
    // Modify the variants to include data required by the practice method
    OpenText(String, String),
    MultipleChoice(String, Vec<String>, usize),
}

impl Exercise {
    // Do not modify this method
    fn practice(&self) {
        match self {
            Self::OpenText(question, answer) => {
                println!("{}", question);
                let input = read_input();
                if input == *answer {
                    println!("Correct!");
                } else {
                    println!("Incorrect!");
                }
            }
            Self::MultipleChoice(question, choices, answer_index) => {
                println!("{}", question);
                for (i, choice) in choices.iter().enumerate() {
                    println!("{}. {}", i + 1, choice);
                }
                let input = read_input();
                let input = input
                    .parse::<usize>()
                    .expect("Invalid input, non-negative integer expected");
                if input == *answer_index + 1 {
                    println!("Correct!");
                } else {
                    println!("Incorrect!");
                }
            }
        }
    }
}

fn main() {
    let exercise1 = Exercise::MultipleChoice(
        "Which of the following is a valid identifier in Rust?".to_string(),
        vec![
            "1abc".to_string(),
            "_abc1".to_string(),
            "abc-1".to_string(),
            "-abc1".to_string(),
        ],
        1,
    );

    let exercise2 = Exercise::OpenText(
        "What is the answer to life, the universe, and everything?".to_string(),
        "42".to_string(),
    );

    exercise1.practice();
    exercise2.practice();
}
