use std::io;
use std::io::Write;

fn main() {
    println!("Hello, world!");
    println!("This is a simple Rust program.");
    println!("Feel free to modify it as you wish.");

    for i in 1..=5 {
        println!("This is message number {}", i);
    }

    print!("Please enter your name: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
        
    println!("Welcome {}!", input.trim());


}
