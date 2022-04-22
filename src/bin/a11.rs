// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct Grocery {
    quantity: i32,
    id: i32,
}

fn print_quantity (item: &Grocery){
    println!("Quantity is: {:?}", item.quantity);
}

fn print_id (item: &Grocery){
    println!("ID is: {:?}", item.id);
}

fn main() {
    let item = Grocery {
        quantity: 21,
        id: 1,
    };
    print_quantity(&item);
    print_id(&item);
}
