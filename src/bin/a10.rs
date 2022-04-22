// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn big_or_small(){
    for i in 100..102 {
        let is_big = i > 100;
        let res = match is_big {
            true => "Its big",
            false => "Its small",
        };
        println!("{} number", res);
    }
}

fn main() {
    big_or_small();
}
