/**
* A simple dice roller for shadowrun 5th edition
**/

use std::io;
use rand::{Rng, thread_rng};

extern crate rand;

fn main() {
    let mut dicepool : i8 = 0;
    let mut input = String::new();

    // read dice pool size, parse to integer
    println!("Please enter your dice pool  size:");

    io::stdin().read_line(&mut input).
        expect("Failed to read :(");

    match input.trim().parse::<i8> () {
        Ok(val) => dicepool = val,
        Err(e) => println!("Please enter a number < 255: {}", e),
    }

    // define thread_rng as randomness source
    let mut rng = thread_rng();

    let mut hits : i8 = 0;
    let mut fails : i8 = 0;

    // roll dices
    for x in 1..dicepool+1 {
        let randnr = rng.gen_range(1, 7);
        if randnr >= 4 {
            hits += 1;
        } else if randnr == 1{
            fails += 1;
        }
        println!("dice nr {} is {}", x, randnr);
    }
    // evaluate dices
    println!("hits: {}  fails: {}", hits, fails);
    if fails > dicepool / 2 {
        if hits == 0 {
            println!("CRITICAL GLITCH!");
        } else {
            println!("GLITCH");
        }
    }
}