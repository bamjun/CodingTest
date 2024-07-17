use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<i32> = input.trim().split_whitespace()
                                 .map(|x| x.parse().expect("Not an integer"))
                                 .collect();

    if numbers.len() == 3 {
        let (a, b, c) = (numbers[0], numbers[1], numbers[2]);
        
        println!("{}", (a + b) % c);
        println!("{}", ((a % c) + (b % c)) % c);
        println!("{}", (a * b) % c);
        println!("{}", (a % c) * (b % c) % c);
    } else {
        eprintln!("Please input exactly three integers");
    }
}
