use std::io;

fn main() {
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("failed to read line");
    let a: i32 = input1.trim().parse().expect("failed to parse number");
    
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("failed to read line");
    let b: i32 = input2.trim().parse().expect("failed to parse number");
    
    if a > 0 && b > 0 {
        println!("1");
    } else if a < 0 && b > 0 {
        println!("2");
    } else if a < 0 && b < 0 {
        println!("3");
    } else {
        println!("4");
    }
}