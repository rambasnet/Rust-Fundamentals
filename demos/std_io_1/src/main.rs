// parse two numbers in one line: 10 3.14

use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter an integer and a float separated by space: ");
    io::stdin().read_line(&mut input).unwrap();

    // split with whitespace
    let mut parts = input.split_whitespace();

    let a: i32 = parts.next().unwrap().parse().unwrap();
    let b: f64 = parts.next().unwrap().parse().unwrap();

    println!("int = {}, float = {}", a, b);
}
