// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn init_vec() -> Vec<i32> {
    vec!(10,20,30,40)
}

fn main() {
    let my_vec = init_vec();
    println!("Today, my vector has {:?} elements", my_vec.len());
    for num in my_vec {
        match num {
            30 => println!("{:#?}","thirty"),
            _ => println!("{:?}", num),
        }
    }
}
