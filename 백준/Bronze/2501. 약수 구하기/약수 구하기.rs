use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i32> = input.split_whitespace().map(|I| I.parse().unwrap()).collect();
    
    let n = nums[0];
    let k = nums[1] as usize;
    let mut list: Vec<i32> = Vec::new();
    
    for i in 1..=n {
        if n % i == 0 {
            list.push(i);
        }
    }
    
    if list.len() >= k {
        println!("{}", list[k-1]);
        
        
    } else {
        println!("0");
    }
    
}