// Topic: Map combinator
//
// Requirements:
// * Given a user name, create and print out a User struct if the user exists
//
// Notes:
// * Use the existing find_user function to locate a user
// * Use the map function to create the User
// * Print out the User struct if found, or a "not found" message if not

#[derive(Debug)]
struct User {
    user_id: i32,
    name: String,
}

/// Locates a user id based on the name.
fn find_user(name: &str) -> Option<i32> {
    let name = name.to_lowercase();
    match name.as_str() {
        "sam" => Some(1),
        "matt" => Some(5),
        "katie" => Some(9),
        _ => None,
    }
}

fn get_users_names() -> Vec<String> {
    vec![
        "sam".to_string(),
        "jam".to_string(),
        "matt".to_string(),
        "bashmatt".to_string(),
        "katrin".to_string(),
        "katie".to_string(),
    ]
}

fn main() {
    let user_name_vec = get_users_names();
    for name in user_name_vec{
        let result = find_user(&name).map(|id|
            User{
                user_id: id,
                name: name.to_string(),
            }
        );
        match result {
            Some(user) => println!("User found! Id: {:?}, name: {:#?}", user.user_id, user.name),
            _ => println!("User with name {} isn't found", name),
        }
    }
}
