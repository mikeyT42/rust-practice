use std::io;
use std::num::ParseFloatError;
use std::process::Command;

const SENTINEL: f32 = -1.0;

enum LoopControl {
    CONTINUE,
    STOP,
}

enum ValidationError {
    OutOfRange,
    Parse(ParseFloatError),
}

impl From<ParseFloatError> for ValidationError {
    fn from(error: ParseFloatError) -> ValidationError {
        ValidationError::Parse(error)
    }
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
        "Enter the amount you spent to two decimal places: the input must be\n\
      between 0 and 1: -1 is to exit.\n"
    );

    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!("Could not read from stdin:: {}", e);
        return LoopControl::STOP;
    }

    let money = match validate(&input) {
        Ok(SENTINEL) => return LoopControl::STOP,
        Ok(input_cash) => input_cash,
        Err(ValidationError::OutOfRange) => {
            eprintln!(
                "You did not provide a number between 1 and 0: \
                        nor was it -1.\n"
            );
            return LoopControl::CONTINUE;
        }
        Err(ValidationError::Parse(float_error)) => {
            eprintln!(
                "There was an error parsing your input:: \n\
                {}",
                float_error
            );
            return LoopControl::CONTINUE;
        }
    };

    let mut num_quarters: i8 = 0;
    let mut num_dimes: i8 = 0;
    let mut num_nickels: i8 = 0;
    let mut num_pennies: i8 = 0;

    calculate_change(
        &money,
        &mut num_quarters,
        &mut num_dimes,
        &mut num_nickels,
        &mut num_pennies,
    );

    println!(
        "\nThe amount you gave was ${:.2}, your change is {} Quarters,\n\
      {} Dimes, {} Nickels, and {} Pennies.\n\n",
        money, num_quarters, num_dimes, num_nickels, num_pennies
    );

    return LoopControl::CONTINUE;
}

// -----------------------------------------------------------------------------
fn validate(input: &str) -> Result<f32, ValidationError> {
    let input = input.trim().parse::<f32>()?;
    if input > 1.0 || (input < 0.0 && input != SENTINEL) {
        return Err(ValidationError::OutOfRange);
    }

    return Ok(input);
}

// -----------------------------------------------------------------------------
fn calculate_change(
    money_dollars: &f32,
    num_quarters: &mut i8,
    num_dimes: &mut i8,
    num_nickels: &mut i8,
    num_pennies: &mut i8,
) {
    let money_cents = money_dollars * 100.0;
    let mut change_left = money_cents as i8;
    *num_quarters = change_left / 25;
    change_left %= 25;
    *num_dimes = change_left / 10;
    change_left %= 10;
    *num_nickels = change_left / 5;
    change_left %= 5;
    *num_pennies = change_left;
}
