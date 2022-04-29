// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

#[derive(Debug)]
enum State {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

fn get_input() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_lowercase().to_string())
}

fn check_state(input_str: &str) -> Option<State> {
    use State::*;
    match input_str {
        "off" => Some(Off),        
        "sleep" => Some(Sleep),
        "rebot" => Some(Reboot),
        "shutdown" => Some(Shutdown),
        "hibernate" => Some(Hibernate),
        _ => None,
    }
}

fn get_answer(state: Option<State>) -> String {
    use State::*;
    match state {
        Some(Off) => String::from("getting off -_-"),
        Some(Sleep) => String::from("...zzzZZZZzzz..."),
        Some(Reboot) => String::from("reloading..."),
        Some(Shutdown) => String::from("shutting down"),
        Some(Hibernate) => String::from("hibernating"),
        _ => String::from("wrong state, try another"),
    }
}

fn main() {
    println!("Enter the power state:");
    let input = get_input().unwrap_or_else(|_| String::new());
    let state = check_state(&input);
    println!("{}", get_answer(state));
}
