use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("n");
    
    let number: i32 = input.trim().parse().expect("n");
    
    let n = number - 543;
    
    println!("{}", n);   
    
}