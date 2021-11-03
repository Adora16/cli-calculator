use std::io::{stdin, stdout};
use std::io::Write;

fn main() {
    let mut user_input = String::new();
    print!("Input: ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input.");
    println!("Your input was: {}", user_input);
}
