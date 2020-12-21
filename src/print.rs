pub fn run() {
    // Print string to console
    println!("Hello from the print.rs file");

    // Basic formatting
    println!("{} is from {}", "Anders", "Aarhus");

    // Positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Anders", "Aarhus", "code"
    );

    // Named arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Baseball"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
