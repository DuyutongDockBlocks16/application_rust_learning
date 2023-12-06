pub fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_owned()
}

// Modify the code from the previous exercise to use c-like struct variants instead of tuple struct variants.
pub enum Exercise {
    OpenText{ question: String,answer: String },
    MultipleChoice{ question: String, choices: Vec<String>, answer_index: usize },
}

impl Exercise {
    fn practice(&self) {
        match self {
            Self::OpenText{ question,answer } => {
                println!("{}", question);
                let input = read_input();
                if input == *answer {
                    println!("Correct!");
                } else {
                    println!("Incorrect!");
                }
            }
            Self::MultipleChoice{
                question,
                choices,
                answer_index,
            } => {
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
    let exercise1 = Exercise::MultipleChoice {
        question: "Which of the following is a valid identifier in Rust?".to_string(),
        choices: vec![
            "1abc".to_string(),
            "_abc1".to_string(),
            "abc-1".to_string(),
            "-abc1".to_string(),
        ],
        answer_index: 1,
    };

    let exercise2 = Exercise::OpenText {
        question: "What is the answer to life, the universe, and everything?".to_string(),
        answer: "42".to_string(),
    };

    exercise1.practice();
    exercise2.practice();
}
