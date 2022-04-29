// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::prelude::*;

fn format_full_time(date: DateTime<Utc>) -> String {
    date.format("%d.%m.%Y-%H:%M:%S").to_string()
}

fn main() {
    let utc: DateTime<Utc> = Utc::now();   
    dbg!(utc);
    println!("{}",format_full_time(utc));
}
