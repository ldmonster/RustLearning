// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

use self::Colors::*;
use std::slice::Iter;

enum Colors {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
}

impl Colors {
    pub fn iterator() -> Iter<'static, Colors> {
        static COLORS: [Colors; 6] = [
            Red,
            Green,
            Blue,
            Yellow,
            Orange,
            Purple,
        ];
        COLORS.iter()
    }
}

fn matcher(color:&Colors){
    match color {
        Colors::Red => println!("{}", "This is Red color"),
        Colors::Green => println!("{}", "This is Green color"),
        Colors::Blue => println!("{}", "This is Blue color"),
        Colors::Yellow => println!("{}", "This is Yellow color"),
        Colors::Orange => println!("{}", "This is Orange color"),
        Colors::Purple => println!("{}", "This is Purple color"),

    }
}

fn main() {
    for color in Colors::iterator() {
        matcher(color);
    }
    
}
