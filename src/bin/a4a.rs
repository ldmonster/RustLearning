// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

fn matcher(is_awailable:bool){
    match is_awailable {
        true => println!("{}", "You're awailable"),
        false => println!("{}", "You're not awailable"),
    }
}

fn main() {
    matcher(true);
    matcher(false);
}
