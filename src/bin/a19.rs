// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

use std::collections::HashMap;

fn get_hashmap() -> HashMap<String, i32> {
    let mut map = HashMap::new();
    map.insert("Chair".to_string(), 5);
    map.insert("Bed".to_string(), 3);
    map.insert("Table".to_string(), 2);
    map.insert("Couche".to_string(), 0);
    map
}

fn main() {
    let _map = get_hashmap();
    let mut total_stock = 0;
    for (key, value) in _map.iter() {
        total_stock += value;
        //  if else variant
        let stock_str = if value == &0 {
            "out of stock".to_string()
        } else {
            format!("in stock {:?}", value)
        };
        //  match variant
        // let stock_str = match value{
        //     0 => "out of stock".to_string(),
        //     _ => format!("in stock {:?}", value),
        // };
        println!("{} {}", key, stock_str);
    }
    println!("\nTotal stock is {:?}", total_stock);
}
