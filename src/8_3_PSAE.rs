use std::io;
use std::str::FromStr;
fn read_line() -> io::Result<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}

fn parse_number(input: &str) -> Result<f32, String> {
    match f32::from_str(input) {
        Ok(number) => Ok(number),
        Err(err) => Err(format!("Failed to parse {} as a number. Error: {}.", input, err)),
    }
}

const ALLOWED_OPERATORS: [char; 4] = ['+', '-', '*', '/'];

fn calculate_from_string(line: &str) -> f32 {
    let mut numbers = Vec::new();
    let mut operators = Vec::new();

    // Remove spaces
    let line = line.replace(|c: char| c.is_whitespace(), "");

    // Split the string into numbers and operators
    let mut parts = line
        .split_inclusive(ALLOWED_OPERATORS)
        // Extract the operator from the string
        .map(|part| {
            if part.ends_with(ALLOWED_OPERATORS) {
                part.split_at(part.len() - 1)
            } else {
                (part, "")
            }
        })
        .flat_map(|(number, operator)| [number, operator])
        .filter(|part| !part.is_empty())
        .collect::<Vec<&str>>();
    if parts.len() % 2 == 0 {
        parts.push("");
    }

    // Parse the numbers and operators
    for (i, part) in parts.iter().enumerate() {
        if i % 2 == 0 {
            // Part is a number
            let number = parse_number(part).expect("Could not parse number");
            numbers.push(number);
        } else {
            // Part is an operator
            operators.push(part);
        }
    }

    // Calculate the result (naively without worrying about operator precedence)
    let mut result = numbers[0];
    for (i, operator) in operators.iter().enumerate() {
        match **operator {
            "+" => result += numbers[i + 1],
            "-" => result -= numbers[i + 1],
            "*" => result *= numbers[i + 1],
            "/" => result /= numbers[i + 1],
            _ => panic!("Unknown operator {operator}"),
        }
    }
    result
}

fn main() {
    let line = read_line().expect("Could not read line");
    let calculation_result = calculate_from_string(&line);
    println!("{}", calculation_result);
}
