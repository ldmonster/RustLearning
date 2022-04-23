// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Person {
    name: String,
    age: i32,
    favorite_color: String,
}
impl Person{
    fn get_persons() -> Vec<Person>{
        vec![
            Person{
                name: "Maxon".to_string(),
                age: 28i32,
                favorite_color: "Red".to_string(),
            },
            Person{
                name: "Lizon".to_string(),
                age: 10i32,
                favorite_color: "Green".to_string(),
            },
            Person{
                name: "Barsik".to_string(),
                age: 3i32,
                favorite_color: "Blue".to_string(),
            },
        ]
    }

    fn print_person(&self){
        println!("Person information:\n\tname: {:#?}\n\tage: {:?}\n\tfavorite color: {:#?}", self.name, self.age, self.favorite_color);
    }
}

fn main() {
    let pers_vec = Person::get_persons();
    for person in pers_vec{
        if !(person.age > 10) {
            person.print_person();
        }
    }
}
