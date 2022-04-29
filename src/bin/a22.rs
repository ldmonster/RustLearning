// Topic: Testing
//
// Requirements:
// * Write tests for the existing program to ensure proper functionality.
//
// Notes:
// * Create at least two test cases for each function.
// * Use `cargo test` to test the program.
// * There are intentional bugs in the program that need to be fixed.
//   * Check the documentation comments for the functions to
//     determine how the they should operate.

/// Ensures n is >= lower and <= upper.
fn clamp(n: i32, lower: i32, upper: i32) -> i32 {
    if n < lower {
        lower
    } else if n > upper {
        upper
    } else {
        n
    }
}

/// Divides a and b.
fn div(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        return None;
    }
    Some(a / b)
}

/// Takes two strings and places them immediately one after another.
fn concat(first: &str, second: &str) -> String {
    format!("{} {}", first, second)
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::{clamp, div, concat};

    #[test]
    fn check_clamp_n(){
        assert_eq!(5, clamp(5, 1, 10), "need to return n");
    }

    #[test]
    fn check_clamp_lower(){
        assert_eq!(5, clamp(1, 5, 10), "need to return lower");
    }

    #[test]
    fn check_clamp_upper(){
        assert_eq!(5, clamp(10, 1, 5), "need to return upper");
    }

    #[test]
    fn check_div(){
        assert_eq!(Some(3), div(6, 2), " 6/2 != 3");
    }

    #[test]
    fn check_div_by_zero(){
        assert_eq!(None, div(6, 0), " divide by zero must return None");
    }

    #[test]
    fn check_concat(){
        assert_eq!(String::from("Borat the Man"), concat("Borat", "the Man"), "bad concat");
    }
}
