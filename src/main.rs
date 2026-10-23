use std::io;

fn main() {
    println!("Hello From gemini code");

    println!("\nPress Enter to exit...");

    // Create a buffer string to capture key presses
    let mut input = String::new();

    // Read input from the terminal console (blocks until Enter is pressed)
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
}