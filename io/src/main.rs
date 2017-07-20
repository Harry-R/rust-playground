use std::io;

fn main() {
    println!("Please enter your Name:");

    let mut name = String::new();

    io::stdin().read_line(&mut name).
        expect("Failed to read :(");

    let name = name.trim();

    println!("Hello {}!", name);
}
