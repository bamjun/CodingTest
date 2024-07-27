use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut n: i32 = input.trim().parse().expect("Failed to parse number");
    n = n / 4;
    let mut output = String::new();
    
    for i in 1..=n {
        output.push_str("long ");
    }
    output.push_str("int");
    println!("{}", output);
    
    
    
}