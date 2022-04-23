// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Tickets{
    Backstage(i32, String),
    Vip(i32, String),
    Standard(i32),
}

fn init_ticket_vec() -> Vec<Tickets> {
    vec![
        Tickets::Backstage(3,"Paul".to_string()),
        Tickets::Vip(2,"Nat".to_string()),
        Tickets::Standard(1),
    ]
}

fn main() {
    let my_vec = init_ticket_vec();
    for ticket in my_vec {
        match ticket {
            Tickets::Backstage(price, name) => println!("Whoa! This is a backstage ticket\n\tprice: {:?}\n\towner name: {}", price, name),
            Tickets::Vip(price, name) => println!("This is a vip ticket\n\tprice: {:?}\n\towner name: {}", price, name),
            Tickets::Standard(price) => println!("This is a junk ticket\n\tprice: {:?}", price),
        };
    }
}
