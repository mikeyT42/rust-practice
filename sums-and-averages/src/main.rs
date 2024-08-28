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
    positive: f64,
    negative: f64,
    overall: f64,
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

    let mut sums = Sums {
        positive: 0.0,
        negative: 0.0,
        overall: 0.0,
    };
    let mut counts = Counts {
        positive: 0,
        negative: 0,
        overall: 0,
    };
    let mut averages = Averages {
        positive: 0.0,
        negative: 0.0,
        overall: 0.0,
    };

    sum_and_count(&numbers, &mut sums, &mut counts);
    average(&sums, &counts, &mut averages);
    print_table(&sums, &counts, &averages);

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

// -----------------------------------------------------------------------------
fn sum_and_count(numbers: &[f64], sums: &mut Sums, counts: &mut Counts) {
    sums.positive = numbers.iter().filter(|n| **n >= 0.0).sum();
    sums.negative = numbers.iter().filter(|n| **n < 0.0).sum();
    sums.overall = numbers.iter().sum();

    counts.positive = numbers.iter().filter(|n| **n >= 0.0).count() as u8;
    counts.negative = numbers.iter().filter(|n| **n < 0.0).count() as u8;
    counts.overall = numbers.iter().count() as u8;
}

// -----------------------------------------------------------------------------
fn average(sums: &Sums, counts: &Counts, averages: &mut Averages) {
    if counts.positive != 0 {
        averages.positive = sums.positive / counts.positive as f64;
    }
    if counts.negative != 0 {
        averages.negative = sums.negative / counts.negative as f64;
    }
    if counts.overall != 0 && sums.overall != 0.0 {
        averages.overall = sums.overall / counts.overall as f64;
    }
}

// -----------------------------------------------------------------------------
fn print_table(sums: &Sums, counts: &Counts, averages: &Averages) {
    println!(
        "\nStatistics:\n\
             {:<18}{:<16}{:<14}\n\
             Positive:{:<9}{:<16.3}{:<14.3}\n\
             Negative:{:<9}{:<16.3}{:<14.3}\n\
             Overall:{:<10}{:<16.3}{:<14.3}\n",
        "Number:",
        "Total:",
        "Average:",
        counts.positive,
        sums.positive,
        averages.positive,
        counts.negative,
        sums.negative,
        averages.negative,
        counts.overall,
        sums.overall,
        averages.overall
    );
}
