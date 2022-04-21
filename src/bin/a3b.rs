// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn desider(num:i32){
    if num > 5 {
        println!("Your number {:?} is more than 5", num);
    } else if num < 5 {
        println!("Your number {:?} is less than 5", num);
    } else {
        println!("Your number {:?} is equal than 5", num);
    } 
}

fn main() {
    desider(1);
    desider(10);
    desider(5);
}
