// Topic: Result
//
// Requirements:
// * Determine if a customer is able to make a restricted purchase
// * Restricted purchases require that the age of the customer
//   is at least 21
//
// Notes:
// * Use a struct to store at least the age of a customer
// * Use a function to determine if a customer can make a restricted purchase
// * Return a result from the function
// * The Err variant should detail the reason why they cannot make a purchase

const AGE_RESTRICTION: i32 = 21;

struct Customer {
    name: String,
    age: i32,
}

fn get_customers() -> Vec<Customer> {
    vec![
        Customer{
            name: "Ivan".to_string(),
            age: 21,
        },
        Customer{
            name: "Masha".to_string(),
            age: 5,
        },
        Customer{
            name: "Sergy".to_string(),
            age: 45,
        },
        Customer{
            name: "Kolokolez".to_string(),
            age: 18,
        },
    ]
}

fn try_purchase(customer: &Customer) -> Result<(), String>{
    if customer.age < AGE_RESTRICTION{ 
        Err(format!("age of customer is {:?}, it's {:?} less than {:?}", customer.age, AGE_RESTRICTION - customer.age, AGE_RESTRICTION).to_string())
    } else {
        Ok(())
    }
}

fn main() {
    let customers_vec = get_customers();
    for customer in customers_vec{
        // match try_purchase(&customer) {
        //     Ok(()) => println!("Purchase for {} is allowed\n\tage:{:?}", customer.name, customer.age),
        //     Err(err) => println!("Purchase for {} not allowed because of:\n\t{}", customer.name, err),
        // }
        let purchased = try_purchase(&customer);
        println!("Result of purchase by {} is {:?}", customer.name, purchased);
    }
}
