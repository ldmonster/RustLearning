// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn loop_example(limit:i32){
    let mut i = 1i32;
    loop {
        println!("Current variable is {:?}", i);
        if i >= limit {
            break;
        }
        i+=1;
    }
}

fn main() {
    loop_example(4);
}
