use std::{
    fs,
    io::{self, BufRead},
};

use rand::seq::SliceRandom;

use crate::problem::{MultipleChoiceProblem, OpenTextProblem, Practice};

pub fn practice_problems<T: Practice + ?Sized>(problems: &[&T]) {
    let mut score = 0;
    for problem in problems {
        let correct = problem.practice();
        if correct {
            score += 1;
        }
    }
    println!("You got {} out of {} correct!", score, problems.len());
}

pub fn practice_problems_random_order<T: Practice + ?Sized>(problems: &[&T]) {
    let mut problems = problems.to_vec();
    problems.shuffle(&mut rand::thread_rng());
    practice_problems(&problems);
}

pub fn read_multiple_choice_problems_from_jsonl_serde(
    filename: &str,
) -> Result<Vec<MultipleChoiceProblem>, std::io::Error> {
    let file = fs::File::open(filename)?;
    let reader = io::BufReader::new(file);
    let mut problems = Vec::new();
    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        };
        let problem: MultipleChoiceProblem = serde_json::from_str(&line)?;
        problems.push(problem);
    }

    Ok(problems)
}

pub fn read_open_text_problems_from_csv_serde(
    filename: &str,
) -> Result<Vec<OpenTextProblem>, std::io::Error> {
    let mut rdr = csv::Reader::from_path(filename)?;
    let mut problems = Vec::new();
    for result in rdr.deserialize() {
        let record: OpenTextProblem = result?;
        problems.push(record);
    }

    Ok(problems)
}
