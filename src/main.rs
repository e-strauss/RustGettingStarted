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
    println!("\n----------------------------------------\n");

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );

    println!("rect1 is {:#?}", rect1);


    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let test_str = "Rust";

    // Call the reverse_string function
    let reversed = reverse_string(test_str);

    // Print the result
    println!("Original: {}", test_str);
    println!("Reversed: {}", reversed);

    dotest("AAAA","TTTT");
    dotest("ATTGC","TAACG");
    dotest("GTAT","CATA");

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{home:?} \n{loopback:?}");
    if let IpAddr::V6(ip) = loopback{
        println!("IPv6 loopback found: {}", ip);
    }
    if let IpAddr::V4(a,b,c,d) = home {
        println!("IPv4 loopback found: {}.{}.{}.{}", a, b, c, d);
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}


fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn reverse_string(s: &str) -> String {
    let mut reversed = String::new();
    for c in s.chars().rev() {
        reversed.push(c);
    }
    reversed
}

fn dna_strand(dna: &str) -> String {
    let mut out: String = String::new();
    for c in dna.chars() {
        out.push(match c {
            'A' => 'T',
            'T' => 'A',
            'C' => 'G',
            'G' => 'C',
            s => s
        });
    }
    out
}

fn dotest(s: &str, expected: &str) {
    let actual = dna_strand(s);
    assert_eq!(actual, expected, "With dna = \"{s}\"\nExpected \"{expected}\" but got \"{actual}\"")
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