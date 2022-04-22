// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum Color {
    Red,
    Green,
    Blue,
}

struct Dimension {
    height: i32,
    width: i32,
    depth: i32,
}

struct Box {
    dimensions: Dimension,
    weight: i32,
    color: Color,
}

impl Color {
    fn get_color(&self) -> &str{
        match self {
            Color::Red => "red",
            Color::Green => "green",
            Color::Blue => "blue",
        }
    }
}

impl Dimension {
    fn create_box(height: i32, width: i32, depth: i32) -> Self {
        Self {
            height: height,
            width: width,
            depth: depth,
        }
    }

    fn print_dimensions(&self){
        println!("\theight: {:?}\n\twidth: {:?}\n\tdepth: {:?}", 
            self.height, 
            self.width, 
            self.depth
        );
    }
}

impl Box {
    
    fn create_box(height: i32, width: i32, depth: i32, weight: i32, color: Color) -> Self {
        Self {
            dimensions: Dimension::create_box(height, width, depth),
            weight: weight,
            color: color,
        }
    }
    
    fn print_box(&self) {
        println!("This box has this characteristics:");
        self.dimensions.print_dimensions();
        println!("\tweight: {:?}\n\tcolor: {}", self.weight, self.color.get_color());
    }
}

fn main() {
    let mut box_vec = Vec::new();
    box_vec.push(Box::create_box(1,1,1,1,Color::Red));
    box_vec.push(Box::create_box(2,2,2,2,Color::Green));
    box_vec.push(Box::create_box(3,3,3,3,Color::Blue));
    for item in box_vec {
        item.print_box();
    }

}
