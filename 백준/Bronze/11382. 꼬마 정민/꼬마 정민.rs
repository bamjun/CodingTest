use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("k");
    
    let n: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("k"))
        .collect();
    
    println!("{}", n[0] + n[1] + n[2]);
}
