use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("j");
    
    let n: Vec<i32> = input.split_whitespace().map(|O| O.parse().expect("J")).collect();
    
    let a = n[0];
    let b = n[1];
    
    if a > b {
        println!(">");
    } else if a < b {
        println!("<");
    } else {
        println!("==");
    }
}