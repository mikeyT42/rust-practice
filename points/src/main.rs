use std::{io, num::ParseIntError, process::Command};

const SENTINEL: &'static str = "\n";

enum ValidationError {
    NoInput,
    TooManyInputs,
    TooLittleInputs,
    Parse(ParseIntError),
}

impl From<ParseIntError> for ValidationError {
    fn from(error: ParseIntError) -> ValidationError {
        ValidationError::Parse(error)
    }
}

enum LoopControl {
    CONTINUE,
    STOP,
}

struct Point {
    x: i32,
    y: i32,
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
        "Please input 2 integers, an x and y value, for a point in \
        space.\n"
    );

    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Could not read from stdin:: {}", e);
        return LoopControl::STOP;
    }
    if input == SENTINEL {
        return LoopControl::STOP;
    }

    let (x, y) = match validate(&input) {
        Ok((x, y)) => (x, y),
        Err(ValidationError::Parse(int_parse_err)) => {
            eprintln!(
                "\nThere was an error parsing your input:: \n\
                {}",
                int_parse_err
            );
            return LoopControl::CONTINUE;
        }
        Err(ValidationError::TooLittleInputs) => {
            eprintln!("\nYou have not provided 2 numbers.\n");
            return LoopControl::CONTINUE;
        }
        Err(ValidationError::TooManyInputs) => {
            eprintln!("\nYou have provided more than 2 numbers.\n");
            return LoopControl::CONTINUE;
        }
        Err(ValidationError::NoInput) => {
            eprintln!("\nYou have provided no input numbers.\n");
            return LoopControl::CONTINUE;
        }
    };

    let point = create_point(x, y);
    let point_string = to_string(&point);
    println!("\n{point_string}\n");

    return LoopControl::CONTINUE;
}

// -----------------------------------------------------------------------------
fn validate(input: &str) -> Result<(i32, i32), ValidationError> {
    let all_input_nums = input
        .lines()
        .next()
        .ok_or_else(|| ValidationError::NoInput)?
        .split_whitespace()
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, ParseIntError>>()?;

    let total_nums = all_input_nums.len();
    if total_nums == 0 {
        return Err(ValidationError::NoInput);
    } else if total_nums < 2 {
        return Err(ValidationError::TooLittleInputs);
    } else if total_nums > 2 {
        return Err(ValidationError::TooManyInputs);
    }

    return Ok((all_input_nums[0], all_input_nums[1]));
}

// -----------------------------------------------------------------------------
fn create_point(x: i32, y: i32) -> Box<Point> {
    Box::new(Point { x, y })
}

// -----------------------------------------------------------------------------
fn to_string(point: &Box<Point>) -> String {
    format!(
        "point {{\n\
        {:>4}: {},\n\
        {:>4}: {}\n\
        }}",
        "x", point.x, "y", point.y
    )
}
