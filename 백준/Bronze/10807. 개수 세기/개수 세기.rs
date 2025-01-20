use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input
        .trim()
        .parse()
        .unwrap();
    input.clear();
    
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|X| X.parse().unwrap())
        .collect();
    input.clear();
    
    io::stdin().read_line(&mut input).unwrap();
    let v: i32 = input.trim().parse().unwrap();
    
    let count = nums.iter().filter(|&&X| X == v).count();
    
    println!("{}", count);
}