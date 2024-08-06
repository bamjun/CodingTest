use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut n: u32 = input.trim().parse().unwrap();
    
    if n == 1 {
        return;
    }
    
    for factor in 2..=(n as f64).sqrt() as u32 {
        while n % factor == 0 {
            println!("{}", factor);
            n /= factor;
        }
       
    }
    
    if n > 1 {
        println!("{}", n);
    }
}