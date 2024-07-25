use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    
    let n: i16 = input.trim().parse().expect("failed to parse number");
    let mut sum: i32 = 0;
    
    for i in 1..=n {
        sum += i as i32;
        
    }
    println!("{}", sum);
    
}