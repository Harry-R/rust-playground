/**
* A simple dice roller for shadowrun 5th edition
**/

use std::io;

fn main() {
    let mut dicepool : i8 = 0;
    let mut input = String::new();

    // read dice pool size, parse to integer
    println!("Please enter your dice pool  size:");

    io::stdin().read_line(&mut input).
        expect("Failed to read :(");

    match input.trim().parse::<i8> () {
        Ok(val) => dicepool = val,
        Err(e) => println!("Please enter a number < 255!"),
    }
    
    println!("Dicpool size is {}.", dicepool);
}
