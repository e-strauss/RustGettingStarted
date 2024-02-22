mod matmul;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::process::exit;
use matmul::dot;

fn main() {
    let a = [1.0,3.0,4.0];
    let b = [3.0,4.0,5.0];
    println!("result: {}", dot(&a, &b))
}

fn intro() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn variable_test() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference : i32 = 8 / 3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    println!("{}", difference)
}