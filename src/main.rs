use std::io;

fn main() {
    print!("Enter your name: ");
    let mut name = String::new();
    io::stdin()
        .read_line(&mut name)
        .expect("Failed to read from standard input.");
    println!("Hello {}!", name);
}
