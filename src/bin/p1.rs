// Project 1: Interactive bill manager
//
// Summary:
//   Create a command line bills/expenses manager that runs
//   interactively. This mini project brings together many of
//   the concepts learn thus far into a single application.
//
//   The user stories/requirements are split into stages.
//   Fully implement each stage as a complete working program
//   before making changes for the next stage. Leverage the
//   compiler by using `cargo check --bin p1` when changing
//   between stages to help identify adjustments that need
//   to be made.
//
// User stories:
// * Stage 1:
//   - I want to add bills, including the name and amount owed.
//   - I want to view existing bills.
// * Stage 2:
//   - I want to remove bills.
// * Stage 3:
//   - I want to edit existing bills.
//   - I want to go back if I change my mind.
//
// Tips:
// * Use the loop keyword to create an interactive menu.
// * Each menu choice should be it's own function, so you can work on the
//   the functionality for that menu in isolation.
// * A vector is the easiest way to store the bills at stage 1, but a
//   hashmap will be easier to work with at stages 2 and 3.

use std::io;
use std::collections::HashMap;

#[derive(Debug)]
struct Bill {
    name: String,
    amount: f64,
}

impl Bill {

    fn new() -> Self {
        Self{
            name: String::new(),
            amount: 0f64,
        }
    } 

    fn get_bill_from_user(&mut self) {
        loop {
            println!("Enter name of bill:");
            let choice = get_input();
            match choice{
                None => {
                    println!("Bill must contain the name, try again!");
                    println!("")
                },
                Some(string) => {
                    self.name = string;
                    break
                },
            }
        }
        loop {
            println!("Enter amount of bill:");
            let choice = get_input();
            match choice{
                None => {
                    println!("Bill must contain amount, try again!");
                    println!("")
                },
                Some(string) => {
                    let amount = string.parse::<f64>();
                    match amount {
                        Err(e) => {
                            println!("Error: {}", e);
                            println!("")
                        },
                        Ok(num) => {
                            if num > 0f64 {
                                self.amount = num;
                                break;
                            }
                            println!("Bill amount must be greater than 0");
                            println!("")
                        },
                    }
                },
            }
        }
    }
}

#[derive(Debug)]
struct Bills {
    counter: i32,
    inner: HashMap<i32, Bill>,
}

impl Bills {

    fn new() -> Self {
        Self{
            counter: 0,
            inner: HashMap::new(),
        }
    } 

    fn add(&mut self, bill: Bill){
        self.counter += 1;
        self.inner.insert(self.counter, bill);
    }

    fn remove(&mut self, id: &i32) -> bool {
        if self.inner.remove(id).is_none() {
            return false;
        }
        if id != &self.counter && id != &0 && self.counter > 1{
            let bill: Bill = self.inner.remove(&self.counter).unwrap();
            self.inner.insert(*id, bill);
        }
        self.counter -=1;
        true
    }

    fn update(&mut self, id: i32, bill: Bill) -> bool {
        self.inner.insert(id, bill).is_some()
    }

    fn len(&mut self) -> usize {
        self.inner.len()
    }

    fn contains_key(&self, key: &i32) -> bool {
        self.inner.contains_key(key)
    }

    fn print_all(&self){
        println!("=================");
        println!("|ID|NAME|AMOUNT|");
        println!("=================");
        let mut vec: Vec<_> = self.inner.iter().collect();
        vec.sort_by(|a, b| a.0.cmp(&b.0));
        vec.iter().for_each(|(key, value)| println!("{:?}. {} - {:?}", key, value.name, value.amount));
    }

}

enum MainMenu {
    AddBill,
    ViewBill,
    RemoveBill,
    UpdateBill,
    Exit,
}

impl MainMenu {
    
    fn show(){
        vec![
            String::from("====Main menu===="),
            String::from("1. Add bills"),
            String::from("2. View bills"),
            String::from("3. Remove bills"),
            String::from("4. Update bills"),
            String::from(""),
            String::from("0. Exit program"),
            String::from("================="),
            String::from("Enter your choice:"),
        ]
        .iter()
        .for_each(|field| println!("{}", field));
    }

    fn from_str(string: &str) -> Option<MainMenu> {
        use MainMenu::*;
        match string {
            "1" => Some(AddBill),
            "2" => Some(ViewBill),
            "3" => Some(RemoveBill),
            "4" => Some(UpdateBill),
            "0" => Some(Exit),
            _ => None,
        }
    }
}

fn get_input() -> Option<String> {
    let mut buffer = String::new();
    while io::stdin().read_line(&mut buffer).is_err(){
        println!("Please enter valid string")
    };

    let input = buffer.trim().to_string();
    if &input == "" {
        return None;
    }
    Some(input)
}

fn get_integer_input() -> i32{
    loop {
        println!("Enter id of bill:");
        let choice = get_input();
        match choice{
            None => {
                println!("Please write id (or 0 to get back to the main menu)!");
                println!("")
            },
            Some(string) => {
                let response: Result<i32, _> = string.parse::<i32>();
                match response {
                    Err(e) => {
                        println!("Error: {}", e);
                        println!("")
                    },
                    Ok(num) => {
                        if num < 0i32 {
                            println!("Please write id (or 0 to get back to the main menu)!");
                            println!("");
                            continue;
                        }
                        return num;
                    },
                }
            }
        }
    }
}

fn main_menu(bills: &mut Bills) -> Option<()>  {
    MainMenu::show();
    loop {
        let choice = get_input().expect("no data entered");
        match MainMenu::from_str(choice.as_str()) {
            Some(MainMenu::AddBill) => add_bill(bills),
            Some(MainMenu::ViewBill) => print_bills(bills),
            Some(MainMenu::RemoveBill) => remove_bill(bills),
            Some(MainMenu::UpdateBill) => update_bill(bills),
            Some(MainMenu::Exit) => return None,
            None => println!("Bad choice"),
        }
        println!("");
        println!("Press enter");
        get_input();
        return Some(());
    }
}

fn print_bills(bills: &mut Bills){
    println!("");
    println!("=================");
    println!("Viewing bills");
    bills.print_all()
}

fn add_bill(bills: &mut Bills) {
    println!("");
    println!("====Add bill====");
    let mut bill = Bill::new();
    bill.get_bill_from_user();
    bills.add(bill); 
    println!("Bill added!");
}

fn remove_bill(bills: &mut Bills) {
    if bills.len() == 0 {
        println!("");
        println!("There's no bills in list");
        return ();
    }
    println!("");
    println!("====Remove bill====");
    bills.print_all();
    println!("");
    println!("If you want to get back to the main menu write 0");
    loop{
        let num = get_integer_input();
        if num == 0i32 {
            println!("Returning to the main menu...");
            return ();
        }
        if bills.remove(&num) {
            break;
        }
        println!("Can't find this id in the list!");
        println!("Please write id (or 0 to get back to the main menu)!");
        println!("");
        continue;
    }
    println!("Bill removed!");
}

fn update_bill(bills: &mut Bills) {
    if bills.len() == 0 {
        println!("");
        println!("There's no bills in list");
        return ();
    }
    loop{
        println!("");
        println!("====Update bill====");
        bills.print_all();
        println!("");
        println!("If you want to get back to the main menu write 0");
        let num = get_integer_input();
        if num == 0i32 {
            println!("Returning to the main menu...");
            return ();
        }
        if !bills.contains_key(&num) {
            println!("Can't find this id in the list!");
            println!("Please write id (or 0 to get back to the main menu)!");
            println!("");
            continue;
        }
        println!("Bill found! Enter new info to update");
        println!("");
        let mut bill = Bill::new();
        bill.get_bill_from_user();
        if bills.update(num, bill) {
            break;
        }
    }
    println!("Bill updated!");
}

fn main() {
    let mut bills = Bills::new();
    bills.add(Bill{name :"kek".to_string(), amount: 25.0});
    bills.add(Bill{name :"skek".to_string(), amount: 55.0});
    bills.add(Bill{name :"baskek".to_string(), amount: 125.0});
    loop{
        match main_menu(&mut bills){
            None => break,
            _ => (),
        }
    }
    println!("Program will be exit")
}
