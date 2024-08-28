use std::{io, num::ParseFloatError, process::Command};

const SENTINEL: &'static str = "\n";

enum ValidationError {
    Parse(ParseFloatError),
    NoInput,
}

impl From<ParseFloatError> for ValidationError {
    fn from(error: ParseFloatError) -> ValidationError {
        ValidationError::Parse(error)
    }
}

struct Sums {
    positive: u64,
    negative: i32,
    overall: i32,
}

struct Counts {
    positive: u8,
    negative: u8,
    overall: u8,
}

struct Averages {
    positive: f64,
    negative: f64,
    overall: f64,
}

enum LoopControl {
    CONTINUE,
    STOP,
}

fn main() {
    let mut clear = Command::new("clear");
    let _clear_status = clear.status().expect("Failed to clear the terminal.");
    println!("---------------------------------------------------------------");
    println!("\tWelcome to the Sentence Data Aggregator.");
    println!("---------------------------------------------------------------");

    let mut l = LoopControl::CONTINUE;
    while match l {
        LoopControl::CONTINUE => true,
        LoopControl::STOP => false,
    } {
        l = input_loop();
    }

    println!("---------------------------------------------------------------");
    println!("\t\tThank you and goodbye.");
    println!("---------------------------------------------------------------");
}

// -----------------------------------------------------------------------------
fn input_loop() -> LoopControl {
    println!(
        "Please input up to floating point or integer numbers. Seperate\n\
      them with spaces. Simply enter a newline character to exit.\n"
    );

    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Could not read from stdin:: {}", e);
        return LoopControl::STOP;
    }
    if input == SENTINEL {
        return LoopControl::STOP;
    }

    let numbers = match validate(&input) {
        Ok(numbers) => numbers,
        Err(ValidationError::Parse(parse_float_error)) => {
            eprintln!(
                "There was an error parsing your input:: \n\
                {}",
                parse_float_error
            );
            return LoopControl::CONTINUE;
        }
        Err(ValidationError::NoInput) => {
            eprintln!("There was no input provided\n");
            return LoopControl::CONTINUE;
        }
    };

    return LoopControl::CONTINUE;
}

// -----------------------------------------------------------------------------
fn validate(input: &str) -> Result<Vec<f64>, ValidationError> {
    let parsed_input_numbers = input
        .lines()
        .next()
        .ok_or_else(|| ValidationError::NoInput)?
        .split_whitespace()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<f64>())
        .collect::<Result<Vec<f64>, ParseFloatError>>()?;

    return Ok(parsed_input_numbers);
}
