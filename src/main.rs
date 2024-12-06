mod matmul;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
use matmul::dot;

fn main() {
    println!("Hi");
    variable_test();
    intro(true);
    let a = [1.0,3.0,4.0];
    let b = [3.0,4.0,5.0];
    println!("result: {}", dot(&a, &b));

    println!("\n----------------------------------------\n");
    println!("{}", first_word(&String::from("helllooooo Mike")));
    println!("{}", first_word(&String::from("Mike")));
    assert_eq!(fake_bin("45385593107843568"), "01011110001100111");
    let result = repeat_str("Hi", 10);
    println!("{result}");
    println!("{}", greet("Greg", "Mike"));
    println!("{}", greet("Mike", "Mike"));
    println!("{}", high_and_low("1 2 3 4 5"));
    println!("{}", high_and_low2("1 2 3 4 5"));
    slices_test();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn high_and_low(numbers: &str) -> String {
    let mut min = i32::MAX;
    let mut max = i32::MIN;
    for i in numbers.split_whitespace() {
        let number = i.parse::<i32>().unwrap();
        if number < min {
            min = number;
        }
        if number > max {
            max = number;
        }
    }
    format!("{} {}", min, max)
}

fn high_and_low2(numbers: &str) -> String {
    use std::cmp;
    let f = |(max, min), x| (cmp::max(max, x), cmp::min(min, x));

    let answer = numbers
        .split_whitespace();
    let answer = answer
        .map(|x| x.parse::<i32>().unwrap());
    let answer = answer
        .fold((i32::MIN, i32::MAX), f);
    format!("{} {}", answer.0, answer.1)
}

fn fake_bin(s: &str) -> String {
    s.chars().map(|c| if c < '5' {'0'} else {'1'}).collect()
}

fn repeat_str(src: &str, count: usize) -> String {
    src.repeat(count)
}

fn greet(name: &str, owner: &str) -> String {
    let greet = if name == owner {
        "boss"
    } else {
        "guest"
    };

    format!("Hello {}", greet)
}

fn slices_test() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn intro(skip : bool) {
    if skip {
        return;
    }
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=1);

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
    let quotient1 : i32 = 8 / 3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    println!("sum: {}, quotient1: {}, product: {}, quotient2: {} truncated: {}, remainder: {}",
             sum, quotient1, product, quotient, truncated, remainder)
}