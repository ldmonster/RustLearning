// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

struct Student {
    name: String,
    locker: Option<i32>,
}

fn get_students() -> Vec<Student>{
    vec![
        Student{
            name: "Ivan".to_string(),
            locker: Some(25),
        },
        Student{
            name: "Lidiya".to_string(),
            locker: Some(3),
        },
        Student{
            name: "George".to_string(),
            locker: None,
        },
    ]
}

fn main() {
    let students_vec = get_students();
    for student in students_vec {
        match student.locker {
            Some(num) => println!("Student:\n\tname: {}\n\tlocker number: {:?}", student.name, num),
            None => println!("Student:\n\tname: {}\n\tlocker number: w/o locker", student.name),
        }
    }
}
