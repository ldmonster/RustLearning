// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

enum Flavors {
    Sweet,
    Hot,
    Sevukha,
}

struct Drink {
    flavor:Flavors,
    value:f32,
    name:&'static str, // mine experiment with string in structures
}

fn get_drinks_data() -> Vec<Drink> {
    let mut vec = Vec::new();
    vec.push(Drink{
        flavor: Flavors::Sweet, 
        value: 0.5, 
        name:"Caluah",
    });
    vec.push(Drink{
        flavor: Flavors::Hot, 
        value: 1.5, 
        name:"Absent",
    });
    vec.push(Drink{
        flavor: Flavors::Sevukha, 
        value: 0.7, 
        name:"Vodka",
    });
    vec
}

fn print_drinks(vec:Vec<Drink>) {
    for drink in vec {
        let flavor:&'static str;
        match drink.flavor {
            Flavors::Sweet => flavor = "Sweet",
            Flavors::Hot => flavor = "Hot",
            Flavors::Sevukha => flavor = "Sevukha",
        }
        println!("Drink:\n name: {}\n value: {:?}\n flavor: {}\n", drink.name, drink.value, flavor);
    }
}

fn main() {
    let vec = get_drinks_data();
    print_drinks(vec);
}
