use std::io;

fn main() {

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|num| num.parse().expect("Please enter a number"))
        .collect();

    let a = nums[0];
    let b = nums[1];
    let c = nums[2];


    let prize = if a == b && b == c {
        10000 + a * 1000
    } else if a == b || a == c {
        1000 + a * 100
    } else if b == c {
        1000 + b * 100
    } else {
        a.max(b).max(c) * 100
    };


    println!("{}", prize);
}
