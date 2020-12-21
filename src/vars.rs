// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language - a variable in a function exists only in the scope of the function

pub fn run() {
    let name = "Anders";
    let mut age = 29;

    println!("My name is {} and I am {}", name, age);

    age = 30;

    println!("My name is {} and I am {}", name, age);

    // Define constant
    // Have to define a type for a constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("Anders", 29);
    println!("{} is {}", my_name, my_age);
}
