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
    let mut input = String::new();

    println!(
        "\n\nPlease input a sentence. If you want to exit, just hit the enter\n\
        key.\n"
    );
    if let Err(error) = io::stdin().read_line(&mut input) {
        eprintln!("Could not read line into input string:: {}", error);
        return LoopControl::STOP;
    }

    if input.is_empty() || input == SENTINEL {
        return LoopControl::STOP;
    }
    let input = input.trim();

    println!("Keystrokes:{:>12}", keystrokes(input));
    println!("Alpa Characters:{:>7}", alph_chars(input));
    println!("Numeric Characters:{:>4}", numeric_chars(input));
    println!("Vowel Characters:{:>6}", vowel_chars(input));

    return LoopControl::CONTINUE;
}

// -----------------------------------------------------------------------------
fn keystrokes(input: &str) -> usize {
    input.chars().count()
}

// -----------------------------------------------------------------------------
fn alph_chars(input: &str) -> usize {
    input
        .chars()
        .filter(|char| char.is_alphabetic())
        .count()
}

// -----------------------------------------------------------------------------
fn numeric_chars(input: &str) -> usize {
    input
        .chars()
        .filter(|char| char.is_numeric())
        .count()
}

// -----------------------------------------------------------------------------
fn vowel_chars(input: &str) -> usize {
    input
        .chars()
        .map(|char| char.to_lowercase().to_string())
        .filter(|lowercase_char| match lowercase_char.as_str() {
            "a" => true,
            "e" => true,
            "i" => true,
            "o" => true,
            "u" => true,
            _ => false,
        })
        .count()
}
