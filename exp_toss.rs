// expected toss till ["TTHT"] <-user input

//to take user input you need std::io
use std::io

fn main() {
    println!("Please enter a sequence of H and T:"); // Prompt the user for input

    let mut user_input = String::new(); // Create a new, mutable String to store the input

    io::stdin() // Get a handle to the standard input stream
        .read_line(&mut user_input) // Read a line from stdin and append it to `user_input`
        .expect("Failed to read line"); // Handle potential errors during input reading

    // The input will include the trailing newline character, so it's often useful to trim it.
    let trimmed_input = user_input.trim();

    println!("Hello, {}!", trimmed_input); // Print the user's input
}