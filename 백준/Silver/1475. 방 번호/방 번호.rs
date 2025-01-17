use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    
    let mut count = [0; 10];
    
    for c in input.chars() {
        let digit = c.to_digit(10).unwrap() as usize;
        count[digit] += 1;
    }
    
    let six_nine_count = count[6] + count[9];
    count[6] = (six_nine_count + 1) / 2;
    count[9] = 0;
    
    let max_set = *count.iter().max().unwrap();
    
    println!("{}", max_set);
    
    
}