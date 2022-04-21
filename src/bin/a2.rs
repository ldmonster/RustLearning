// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn sum(first:i32, second:i32) -> i32 {
    first + second
}

fn display<T: std::fmt::Display + std::fmt::Debug>(input:T, header:&str) {
    println!("My {} is: {:?}", header, input);
}

fn main() {
    display(sum(1,2), "sum");
    display("sum(1,2)", "string");
}
