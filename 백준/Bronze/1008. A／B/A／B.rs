use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin().read_line(&mut input).expect("N");
    
    let numbers: Vec<f64> = input.split_whitespace().map(|N| N.parse().expect("N")).collect();

    let div = numbers[0] / numbers[1];
    
    println!("{}", div);
    
}