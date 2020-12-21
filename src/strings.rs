// Primitive str = Immutable fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data
// Both are just a grouping of u8's

#[allow(unused_variables)]
pub fn run() {
    // Primitive string/string slice
    let _hello_str: &str = "Hello";

    // Mutable String
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {}", hello.len());

    // Push on a char
    hello.push('W');

    // Push on a string
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World': {}", hello.contains("World"));

    // Replace
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", hello);

    // Translate between String and str
    let example_str: &str = "Howdy";
    let example_string: String = String::from("Partner");
    let string_from_str: String = example_str.to_string();

    // Is really string slices stored in the programs binary
    let string_literal: String = "Some hardcoded string".to_string();

    let string_from_hardcoded = String::from("Some hardcoded");
    let string_from_str_var = String::from(example_str);

    // & is called a de-ref symbol
    // Rust knows how to convert a String into a &str by looking at the referenced &example_string
    let str_from_string: &str = &example_string;

    // Combining strings
    let combine_string_literals = ["first", "second"]; // Results in String
    let combine_with_format_macro = format!("{} {}", "first", "second"); // Results in String

    // Only works if example_string is first - because of borrowing
    // let string_plus_str = example_string + example_str;

    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push_str("Some hardcoded literal");
    mut_string.push('m'); // .push is for chars

    let a = String::from("a");
    let b = String::from("b");
    // Using & converts the String to a string slice, making it possible to append
    let combined = a + &b;

    let str_from_substring: &str = &example_string[..2];
}
