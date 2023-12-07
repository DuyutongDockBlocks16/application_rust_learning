use application::problem::Practice;
use application::quiz::{
    practice_problems_random_order, read_multiple_choice_problems_from_jsonl_serde,
    read_open_text_problems_from_csv_serde,
};

fn read_problems() -> Result<Vec<Box<dyn Practice>>, std::io::Error> {
    let mcq_problems =
        read_multiple_choice_problems_from_jsonl_serde("src/resources/mcq-problems.jsonl")?;

    let open_problems =
        read_open_text_problems_from_csv_serde("src/resources/open-text-problems.csv")?;

    let mut problems: Vec<Box<dyn Practice>> = Vec::new();

    // Add MCQ problems to the vector
    for problem in mcq_problems {
        problems.push(Box::new(problem));
    }

    // Add open text problems to the vector
    for problem in open_problems {
        problems.push(Box::new(problem));
    }

    Ok(problems)
}

fn main() -> Result<(), std::io::Error> {
    let problems = read_problems()?;
    let problems: Vec<&dyn Practice> = problems.iter().map(|p| p.as_ref()).collect();
    practice_problems_random_order(&problems);
    Ok(())
}
