use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    
    io::stdin().read_line(&mut input1).expect("n");
    io::stdin().read_line(&mut input2).expect("n");
    
    let a: i32 = input1.trim().parse().expect("N");
    let b: i32 = input2.trim().parse().expect("N");
    
    let b1 = b % 10;
    let b2 = (b / 10) % 10;
    let b3 = b / 100;
    
    println!("{}", a*b1);
    println!("{}", a*b2);
    println!("{}", a*b3);
    println!("{}", a*b);
    
    
}