use crate::util::read_input;
use serde::Deserialize;

pub trait Practice {
    fn practice(&self) -> bool;
    fn correct_answer(&self) -> &str;
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct OpenTextProblem {
    question: String,
    correct_answer: String,
}

impl Practice for OpenTextProblem {
    fn correct_answer(&self) -> &str {
        &self.correct_answer
    }
    fn practice(&self) -> bool {
        let Self {
            question,
            correct_answer,
        } = self;
        // Ask the question
        println!("{question}");
        let user_answer = read_input();
        // Check the answer
        let correct = &user_answer == correct_answer;
        if correct {
            println!("Correct!");
        } else {
            println!("Incorrect! The correct answer is {correct_answer}");
        }
        correct
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct MultipleChoiceProblem {
    question: String,
    choices: Vec<String>,
    correct_index: u8,
}

impl Practice for MultipleChoiceProblem {
    fn correct_answer(&self) -> &str {
        &self.choices[self.correct_index as usize]
    }
    fn practice(&self) -> bool {
        // Destructure fields to variables
        let Self {
            question,
            choices,
            correct_index,
        } = self;

        // Print the question and choices
        println!("{question}");
        for (i, choice) in choices.iter().enumerate() {
            println!("{}. {}", i + 1, choice);
        }

        // Read the user's answer until it's valid or stack overflow
        fn prompt_answer(choices: &[String]) -> u8 {
            let user_answer = read_input();
            let user_answer = user_answer.parse::<u8>().unwrap_or(0);
            if user_answer == 0 || user_answer as usize > choices.len() {
                println!("Please enter a number between 1 and {}", choices.len());
                prompt_answer(choices)
            } else {
                user_answer
            }
        }

        let user_answer = prompt_answer(choices);

        // Check the answer
        let correct = user_answer == *correct_index + 1;
        if correct {
            println!("Correct!");
        } else {
            println!("Incorrect! The correct answer is {}", *correct_index + 1);
        }
        correct
    }
}
