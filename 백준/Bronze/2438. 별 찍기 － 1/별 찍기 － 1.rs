use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Failed to parse number");
    
    for i in 1..=n {
        println!("{}", "*".repeat(i));
    }
    
}