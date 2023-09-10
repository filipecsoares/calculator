use std::io;

fn main() {
    println!("Welcome to Rust calculator!\n");
    println!("Enter the mathematical expression:");
    let mut expression = String::new();
    io::stdin().read_line(&mut expression).expect("Failed to read line");
    println!("Result: {}", expression);
}