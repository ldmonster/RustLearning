// Topic: Looping using the while statement
//
// Program requirements:
// * Counts down from 5 to 1, displays the countdown
//   in the terminal, then prints "done!" when complete.
//
// Notes:
// * Use a mutable integer variable
// * Use a while statement
// * Print the variable within the while loop
// * Do not use break to exit the loop

fn loop_example(limit:i32){
    let mut i = limit;
    while i >= 1 {
        println!("Current variable is {:?}", i);
        i-=1;
    }
    println!("{}", "Done");
}

fn main() {
    loop_example(5);
}