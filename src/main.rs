use std::io::{self, Write};

fn main() {
    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read from standard input.");
    let name = name.trim();
    println!("Hello {}!", name);
}
