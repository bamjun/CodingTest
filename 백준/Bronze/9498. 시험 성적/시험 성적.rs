use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let n: i32 = input.trim().parse().expect("Failed to parse number");
    
    if n >= 90 {
        println!("A");
    } else if n >= 80 {
        println!("B");
    } else if n >= 70 {
        println!("C");
    } else if n >= 60 {
        println!("D");
    } else {
        println!("F");
    }
}