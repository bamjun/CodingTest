use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");
    let n: i32 = input.trim().parse().expect("failed to parse number");
    
    for i in 0..n {
        let mut input2 = String::new();
        io::stdin().read_line(&mut input2).expect("failed to read line");
        let numbers: Vec<u8> = input2.trim().split_whitespace().map(|O| O.parse().expect("failed")).collect();
        
        println!("{}", numbers[0] + numbers[1]);
    }
    
    
}