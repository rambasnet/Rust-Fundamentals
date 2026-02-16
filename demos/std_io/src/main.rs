use std::io;

fn main() {
    let mut input = String::new(); // Create a mutable String to hold the user input

    println!("Please enter some input:"); // Prompt the user for input

    io::stdin()
        .read_line(&mut input) // Read a line of input from the user and store it in the `input` variable
        .expect("Failed to read line"); // Handle any potential errors that may occur during input reading

    println!("You entered: {}", input); // Print the input back to the user
}
