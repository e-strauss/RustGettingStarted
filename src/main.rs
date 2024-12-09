mod matmul;

use rand::Rng;
use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
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

    assert_eq!(sum_two_smallest_numbers(&[5, 8, 12, 19, 22]),  13, "Incorrect result for [5, 8, 12, 19, 22]");
    assert_eq!(sum_two_smallest_numbers(&[15, 28, 4, 2, 43]), 6, "Incorrect result for [15, 28, 4, 2, 43]");
    assert_eq!(sum_two_smallest_numbers(&[23, 71, 33, 82, 1]), 24, "Incorrect result for [23, 71, 33, 82, 1]");
    assert_eq!(sum_two_smallest_numbers(&[52, 76, 14, 12, 4]), 16, "Incorrect result for [52, 76, 14, 12, 4]");
    assert_eq!(sum_two_smallest_numbers(&[1, 1, 5, 5]),  2, "Incorrect result for [1, 1, 5, 5]");

    assert_eq!(find_even_index(&[1, 2, 3, 4, 3, 2, 1]), Some(3));
    assert_eq!(find_even_index(&[1, 100, 50, -51, 1, 1]), Some(1));
    assert_eq!(find_even_index(&[1, 2, 3, 4, 5, 6]), None);
    assert_eq!(find_even_index(&[20, 10, 30, 10, 10, 15, 35]), Some(3));

    // assert_eq!(
    //     find_deleted_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9], &[3, 2, 4, 6, 7, 8, 1, 9]),
    //     Some(5)
    // );
    // assert_eq!(
    //     find_deleted_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9], &[3, 2, 4, 6, 7, 8, 9, 5]),
    //     Some(1)
    // );
    // assert_eq!(
    //     find_deleted_number(&[1, 2, 3, 4, 5, 6, 7, 8, 9], &[3, 2, 4, 1, 7, 8, 9, 5]),
    //     Some(6)
    // );

    let mut cache = LRUCache::new(2);

    cache.put(1, "one");
    cache.put(2, "two");
    println!("{:?}", cache.get(1)); // Some("one")
    cache.put(3, "three");          // Evicts key 2
    println!("{:?}", cache.get(2)); // None
    cache.put(4, "four");           // Evicts key 1
    println!("{:?}", cache.get(1)); // None
    println!("{:?}", cache.get(3)); // Some("three")
    println!("{:?}", cache.get(4)); // Some("four")

    assert_eq!(likes(&[]), "no one likes this");
    assert_eq!(likes(&["Peter"]), "Peter likes this");
    assert_eq!(likes(&["Jacob", "Alex"]), "Jacob and Alex like this");
    assert_eq!(
        likes(&["Max", "John", "Mark"]),
        "Max, John and Mark like this"
    );
    assert_eq!(
        likes(&["Alex", "Jacob", "Mark", "Max"]),
        "Alex, Jacob and 2 others like this"
    );
}

fn likes(names: &[&str]) -> String {
    match names.len() {
        0 => "no one likes this".to_string(),
        1 => format!("{} likes this", names[0]),
        2 => format!("{} and {} like this", names[0], names[1]),
        3 => format!("{}, {} and {} like this", names[0], names[1], names[2]),
        l => format!("{}, {} and {} others like this", names[0], names[1], l - 2),
    }
}

struct LRUCache<K,V> {
    map: HashMap<K, V>,
    queue: VecDeque<K>,
    capacity: usize,
}

impl<K, V> LRUCache<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    fn new(capacity: usize) -> Self {
        Self {
            map: HashMap::new(),
            queue: VecDeque::new(),
            capacity,
        }
    }

    fn get(&mut self, key: K) -> Option<V> {
        if let Some(value) = self.map.get(&key) {
            self.queue.retain(|item| item != &key);
            self.queue.push_back(key.clone());
            Some(value.clone())
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.map.insert(key.clone(), value);
            self.queue.retain(|item| item != &key);
            self.queue.push_back(key);
        } else {
            if self.map.len() == self.capacity {
                if let Some(least_used_key) = self.queue.pop_front() {
                    self.map.remove(&least_used_key);
                }
            }
            // Insert the new key-value pair
            self.map.insert(key.clone(), value);
            self.queue.push_back(key);
        }
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

fn sum_two_smallest_numbers(numbers: &[u32]) -> u32 {
    let mut min1 = u32::MAX;
    let mut min2 = u32::MAX;
    for &number in numbers {
        match (number < min2, number < min1) {
            (true, true) => {min2 = min1; min1 = number;},
            (true, false) => min2 = number,
            _ => {}
        }
    }
    min1 + min2
}

fn find_even_index(arr: &[i32]) -> Option<usize> {
    for i in 0..arr.len() {
        let left: i32 = arr[0..i].iter().sum();
        let right = arr[(i+1)..].iter().sum();
        if left == right {
            return Some(i);
        }
    }
    None
}

fn find_deleted_number(list: &[u16], mixed_list: &[u16]) -> Option<u16> {

    todo!()
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