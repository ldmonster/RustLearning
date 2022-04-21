// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn matcher(num:i32){
    match num {
        0 => println!("{}", "Zero"),
        1 => println!("{}", "One"),
        2 => println!("{}", "Two"),
        3 => println!("{}", "Three"),
        4 => println!("{}", "Four"),
        5 => println!("{}", "Five"),
        6 => println!("{}", "Six"),
        7 => println!("{}", "Seven"),
        8 => println!("{}", "Eight"),
        9 => println!("{}", "Nine"),
        _ => println!("{}", "More than 9"),
    }
}

fn main() {
    for i in 0..=10{
        matcher(i);
    }
}
