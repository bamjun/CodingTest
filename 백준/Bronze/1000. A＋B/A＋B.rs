use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("n");

    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("n"))
        .collect();

    let sum = numbers[0] + numbers[1];

    println!("{}", sum);
}
