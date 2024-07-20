use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("N");
    
    let numbers: Vec<i32> = input.split_whitespace().map(|N| N.parse().expect("N")).collect();
    
    let A = numbers[0];
    let B = numbers[1];
    let C = numbers[2];
    
    println!("{}", (A+B)%C);
    println!("{}", ((A%C) + (B%C))%C);
    println!("{}", (A*B)%C);
    println!("{}", ((A%C)*(B%C))%C);
    
}