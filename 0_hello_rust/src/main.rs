
/*
    0_hello_rust/src/main.rs
*/

// external crate: rand
extern crate rand;

// import input/output (io) library
// from standard (std) library
use std::io;
// import random library
use rand::Rng;

use std::cmp::Ordering;

// main function
fn main() {
    // print message to console out
    println!("\nGuess the number!");

    // generate a random number
    let secret_num = rand::thread_rng().gen_range(1, 101);

    // infinite loop
    loop {
        // ask user for input
        println!("\nInput your guess: ");
        // create mutable string
        let mut guess = String::new();
        // get user input from standard in
        io::stdin().read_line(&mut guess)
                   .expect("Failed to read line!");
        // ensure user input is a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_)  => {
                println!("Please enter a number!");
                continue;
            }
        };
        // guess.trim().parse().expect("Please enter a number!");

        // print random number and user input
        println!("\nThe secret number is: {}", secret_num);
        println!("\nYou guessed: {}", guess);

        // check user input against secret
        match guess.cmp(&secret_num){
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You Win!!!");
                break;
            }
        }
    }
}

/* EOF */
