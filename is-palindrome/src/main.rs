use std::io;
use std::process::Command;

const SENTINEL: &'static str = "\n";

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
        "Please enter a string that is a palindrome; if you want to exit then\n\
       then just hit enter. It can be a sentence or a word.\n"
    );
    let mut input = String::new();
    if let Err(error) = io::stdin().read_line(&mut input) {
        eprintln!("Could not read line into input string:: {}", error);
        return LoopControl::STOP;
    }

    if input.is_empty() || input == SENTINEL {
        return LoopControl::STOP;
    }
    let cleaned_input = match clean_input(&mut input) {
        Some(cleaned_input) => cleaned_input,
        None => {
            eprintln!("The string is empty.");
            return LoopControl::CONTINUE;
        }
    };

    if is_palindrome(cleaned_input) {
        println!("\nThe string you entered is a palindrome.\n");
    } else {
        println!("\nThe string you entered is not a palindrome.\n");
    }

    return LoopControl::CONTINUE;
}

// -----------------------------------------------------------------------------
fn clean_input(input: &str) -> Option<String> {
    input
        .chars()
        .filter(|char| char.is_alphanumeric())
        .map(|char| char.to_lowercase().to_string())
        .reduce(|acc, char| String::from(acc) + &char)
}

// -----------------------------------------------------------------------------
fn is_palindrome(input: String) -> bool {
    let forward = input.chars();
    let backward = input.chars().rev();

    forward.eq(backward)
}
