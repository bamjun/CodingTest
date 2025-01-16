use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let a: u32 = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let b: u32 = input.trim().parse().unwrap();
    
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    let c: u32 = input.trim().parse().unwrap();
    
    let result = a * b * c;
    
    let mut count = [0; 10];
    
    for digit in result.to_string().chars() {
        if let Some(index) = digit.to_digit(10) {
            count[index as usize] += 1;
        }
    }
    
    for count in &count {
        println!("{}", count);
    }
    
}