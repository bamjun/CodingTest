use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("N");
    
    let numbers: Vec<i32> = input.split_whitespace().map(|N| N.parse().expect("N")).collect();
    
    let a = numbers[0];
    let b = numbers[1];
    
    println!("{}", a+b);
    println!("{}", a-b);
    println!("{}", a*b);
    println!("{}", a/b);
    println!("{}", a%b);
    
}