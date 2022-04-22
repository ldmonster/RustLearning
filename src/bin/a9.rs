// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

fn get_coords(modifier:i32) -> (i32,i32){
    (5,5+modifier)
}

fn main() {
    let mut cycle = 1i32;
    for i in -1..2 {
        let (x,y) = get_coords(i);
        println!("Cycle number: {:?}, y modifier: {:?}", cycle, i);
        println!("x: {:?}", x);
        if y > 5 {
            println!("y: {:?} >  5", y);
        } else if y < 5 {
            println!("y: {:?} <  5", y);
        } else {
            println!("y: {:?} =  5", y);
        }
        cycle += 1;
    }
}
