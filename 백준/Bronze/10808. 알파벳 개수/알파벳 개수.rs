use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    
    let mut counts = vec![0; 26];
    
    for c in input.chars() {
        let index = (c as usize) - ('a' as usize);
        counts[index] += 1;
    }
    
    let output: Vec<String> = counts.iter().map(|&count| count.to_string()).collect();
    println!("{}", output.join(" "));
}