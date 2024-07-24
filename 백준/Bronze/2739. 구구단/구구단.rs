use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let n: u32 = input.trim().parse().expect("failed to parse number");
    
    for i in 1..=9 {
        println!("{} * {} = {}", n, i, n*i);
    }
}